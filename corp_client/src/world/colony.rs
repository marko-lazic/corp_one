use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_mod_picking::backends::rapier::RapierPickTarget;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::RaycastMesh;
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
use serde::Deserialize;

use crate::asset::asset_loading::{MaterialAssets, SceneAssets};
use crate::input::Ground;
use crate::state::Despawn;
use crate::world::colony::barrier::{BarrierControl, BarrierField, BarrierPlugin};
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::vortex::{VortexGate, VortexNode, VortexPlugin};
use crate::world::colony::zone::Zone;
use crate::world::{physics, WorldSystemSet};
use crate::Game;
use crate::GameState;

mod asset;
pub mod barrier;
pub mod colony_assets;
pub mod intractable;
mod scene_hook;
pub mod vortex;
pub mod zone;

#[derive(Debug, Deserialize, Clone)]
pub enum Colony {
    Cloning,
    Iris,
    Liberte,
    Playground,
}

impl Default for Colony {
    fn default() -> Self {
        Self::Cloning
    }
}

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VortexNode>();
        app.register_type::<VortexGate>();
        app.register_type::<BarrierField>();
        app.register_type::<BarrierControl>();
        app.add_plugin(RonAssetPlugin::<ColonyAsset>::new(&["colony"]));
        app.add_plugin(VortexPlugin);
        app.add_plugins(DefaultPickingPlugins);
        app.add_plugin(BarrierPlugin);
        app.add_plugin(HookPlugin);
        app.add_systems(
            (
                Self::setup_colony,
                Self::setup_debug_plane,
                Self::setup_zones,
            )
                .chain()
                .in_schedule(OnEnter(GameState::LoadColony)),
        );

        app.add_system(
            Self::next_state_spawn_player
                .in_set(OnUpdate(GameState::LoadColony))
                .run_if(Self::is_colony_loaded),
        );
        app.add_system(
            Self::next_state_playing
                .in_set(OnUpdate(GameState::SpawnPlayer))
                .after(WorldSystemSet::CameraSetup),
        );

        app.add_system(Self::update_lights.in_schedule(OnEnter(GameState::Playing)));
    }
}

impl ColonyPlugin {
    fn setup_colony(
        colony_assets: Res<Assets<ColonyAsset>>,
        scene_assets: Res<SceneAssets>,
        mut commands: Commands,
        game: Res<Game>,
    ) {
        info!("Setup colony");
        let current_colony = colony_assets.get(&game.current_colony_asset).unwrap();
        let colony_scene = match current_colony.name {
            Colony::Cloning => scene_assets.cloning.clone(),
            Colony::Iris => scene_assets.iris.clone(),
            Colony::Liberte => scene_assets.liberte.clone(),
            _ => scene_assets.liberte.clone(),
        };

        commands.spawn((
            HookedSceneBundle {
                scene: SceneBundle {
                    scene: colony_scene,
                    ..default()
                },
                hook: SceneHook::new(|entity, commands| {
                    if let Some(name) = entity.get::<Name>().map(|t| t.as_str()) {
                        scene_hook::scene_hook_insert_components(name, commands)
                    }
                }),
            },
            Despawn,
        ));
    }

    fn setup_debug_plane(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // spawn cube
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_xyz(10., 0., 0.),
                ..Default::default()
            },
            Collider::cuboid(2.6, 2.6, 2.6),
            PickableBundle::default(),
            RapierPickTarget::default(),
            OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>| {
                info!("Clicked on entity {:?}", event.target);
                Bubble::Up
            }),
            Despawn,
        ));

        info!("Setup debug plane");
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane {
                    size: 100.0,
                    ..default()
                })),
                transform: Transform::from_translation(Vec3::new(4., -0.01, 4.)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 0.0,
                    reflectance: 0.0,
                    metallic: 0.0,
                    ..Default::default()
                }),
                ..Default::default()
            },
            RigidBody::Fixed,
            Collider::cuboid(100.0, 0.01, 100.0),
            RaycastMesh::<Ground>::default(),
            Despawn,
        ));
    }

    fn setup_zones(
        mut commands: Commands,
        game: Res<Game>,
        assets: Res<Assets<ColonyAsset>>,
        material_assets: Res<MaterialAssets>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        info!("Setup zones");
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            for zone_asset in &colony_asset.zones {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane {
                            size: zone_asset.size,
                            ..default()
                        })),
                        transform: Transform::from_translation(zone_asset.position),
                        material: material_assets.get_material(&zone_asset.material),
                        ..Default::default()
                    },
                    Sensor,
                    Collider::cuboid(0.5, 1.0, 0.5),
                    Zone::from(*zone_asset),
                    physics::CollideGroups::zone(),
                    Despawn,
                ));
            }
        }
    }

    fn is_colony_loaded(vortex_nodes: Query<Entity, With<VortexNode>>) -> bool {
        vortex_nodes.iter().count() > 0
    }

    fn next_state_spawn_player(mut next_state: ResMut<NextState<GameState>>) {
        info!("State: Spawn Player");
        next_state.set(GameState::SpawnPlayer);
    }

    fn next_state_playing(mut next_state: ResMut<NextState<GameState>>) {
        info!("State: Playing");
        next_state.set(GameState::Playing);
    }

    // Temporary fixes the problem with shadows not working
    fn update_lights(mut query: Query<&mut PointLight>) {
        info!("Update lights");
        for mut point_light in query.iter_mut() {
            point_light.shadows_enabled = true;
        }
    }
}
