use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use bevy_mod_raycast::RaycastMesh;
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};
use serde::Deserialize;

use crate::asset::asset_loading::{MaterialAssets, SceneAssets};
use crate::constants::state::GameState;
use crate::input::Ground;
use crate::world::colony::barrier::{BarrierControl, BarrierField, BarrierPlugin};
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::vortex::{VortexGate, VortexNode, VortexPlugin};
use crate::world::colony::zone::Zone;
use crate::world::{physics, WorldSystem};
use crate::Game;

mod asset;
pub mod barrier;
pub mod colony_assets;
pub mod intractable;
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
        app.add_enter_system(GameState::LoadColony, Self::setup_colony);
        app.add_enter_system(GameState::LoadColony, Self::setup_debug_plane);
        app.add_enter_system(GameState::LoadColony, Self::setup_zones);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::LoadColony)
                .run_if(Self::is_colony_loaded)
                .with_system(Self::next_state_spawn_player)
                .into(),
        );
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::SpawnPlayer)
                .after(WorldSystem::CameraSetup)
                .with_system(Self::next_state_playing)
                .into(),
        );

        app.add_enter_system(GameState::Playing, Self::update_lights);
        app.add_exit_system(GameState::Playing, Self::teardown_entities);
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

        commands.spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: colony_scene.clone(),
                ..default()
            },
            hook: SceneHook::new(|entity, commands| {
                match entity.get::<Name>().map(|t| t.as_str()) {
                    Some("VortexGate") => commands.insert((
                        VortexGate,
                        Sensor,
                        Collider::cuboid(0.5, 1.0, 0.5),
                        physics::CollideGroups::vortex_gate(),
                    )),
                    Some("VortexNode1") | Some("VortexNode2") | Some("VortexNode3")
                    | Some("VortexNode4") | Some("VortexNode5") | Some("VortexNode6") => {
                        commands.insert(VortexNode)
                    }
                    Some("BarrierField1") => commands.insert(BarrierField::new("B1")),
                    Some("BarrierControl11") | Some("BarrierControl12") => {
                        info!("Barrier access {:?}", entity.get::<Transform>());
                        commands.insert((BarrierControl::new("B1"), PickableBundle::default()))
                    }
                    Some("BarrierField2") => commands.insert(BarrierField::new("B2")),
                    Some("BarrierControl21") | Some("BarrierControl22") => {
                        commands.insert((BarrierControl::new("B2"), PickableBundle::default()))
                    }
                    _ => commands,
                };
            }),
        });
    }

    fn setup_debug_plane(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        info!("Setup debug plane");
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
                transform: Transform::from_translation(Vec3::new(4., -0.01, 4.)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 0.0,
                    reflectance: 0.0,
                    metallic: 0.0,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert((
                RigidBody::Fixed,
                Collider::cuboid(100.0, 0.01, 100.0),
                RaycastMesh::<Ground>::default(),
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
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane {
                            size: zone_asset.size.clone(),
                        })),
                        transform: Transform::from_translation(zone_asset.position),
                        material: material_assets.get_material(&zone_asset.material),
                        ..Default::default()
                    })
                    .insert((
                        Sensor,
                        Collider::cuboid(0.5, 1.0, 0.5),
                        Zone::from(zone_asset.clone()),
                        physics::CollideGroups::zone(),
                    ));
            }
        }
    }

    fn is_colony_loaded(vortex_nodes: Query<Entity, With<VortexNode>>) -> bool {
        if vortex_nodes.iter().count() > 0 {
            true
        } else {
            false
        }
    }

    fn next_state_spawn_player(mut commands: Commands) {
        info!("State: Spawn Player");
        commands.insert_resource(NextState(GameState::SpawnPlayer));
    }

    fn next_state_playing(mut commands: Commands) {
        info!("State: Playing");
        commands.insert_resource(NextState(GameState::Playing));
    }

    // Temporary fixes the problem with shadows not working
    fn update_lights(mut query: Query<&mut PointLight>) {
        info!("Update lights");
        for mut point_light in query.iter_mut() {
            point_light.shadows_enabled = true;
        }
    }

    fn teardown_entities(mut commands: Commands, entities: Query<Entity>) {
        info!("Teardown entities");
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
