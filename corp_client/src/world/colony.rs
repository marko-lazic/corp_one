use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, RayCastSource};
use bevy_mod_raycast::RayCastMesh;
use heron::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};
use serde::Deserialize;

use crate::asset::asset_loading::{MaterialAssets, SceneAssets};
use crate::constants::state::GameState;
use crate::input::MyRayCastSet;
use crate::world::camera::TopDownCamera;
use crate::world::colony::barrier::{BarrierAccess, BarrierField, BarrierPlugin};
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::vortex::{VortexGate, VortexNode, VortexPlugin};
use crate::world::colony::zone::{Zone, ZoneEntities};
use crate::world::WorldSystem;
use crate::Game;

mod asset;
pub mod barrier;
pub mod colony_assets;
pub mod intractable;
pub mod vortex;
pub mod zone;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PhysicsLayer)]
pub enum Layer {
    VortexGate,
    Zone,
    Player,
}

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

impl ColonyPlugin {
    fn setup_debug_plane(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
                transform: Transform::from_translation(Vec3::new(4., -0.2, 4.)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 0.0,
                    reflectance: 0.0,
                    metallic: 0.0,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(RayCastMesh::<MyRayCastSet>::default());
    }

    fn setup_zones(
        mut commands: Commands,
        game: Res<Game>,
        assets: Res<Assets<ColonyAsset>>,
        material_assets: Res<MaterialAssets>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            for zone_asset in &colony_asset.zones {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane {
                            size: zone_asset.size.clone(),
                        })),
                        transform: Transform::from_translation(zone_asset.position),
                        material: material_assets.get_material(&zone_asset.material),
                        ..Default::default()
                    })
                    .insert(RigidBody::Sensor)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(0.5, 1.0, 0.5),
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layer::Zone)
                            .with_mask(Layer::Player),
                    )
                    .insert(Zone::from(zone_asset.clone()))
                    .insert(ZoneEntities::default());
            }
        }
    }

    fn vortex_gate_insert(mut commands: Commands, query: Query<Entity, Added<VortexGate>>) {
        for gate in query.iter() {
            commands
                .entity(gate)
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(0.5, 1.0, 0.5),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(Layer::VortexGate)
                        .with_mask(Layer::Player),
                );
        }
    }

    fn barrier_access_insert(mut commands: Commands, query: Query<Entity, Added<BarrierAccess>>) {
        for gate in query.iter() {
            commands
                .entity(gate)
                .insert_bundle(PickableBundle::default());
        }
    }

    fn setup_camera(mut commands: Commands) {
        commands
            .spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_translation(Vec3::new(-3.0, 3.0, 5.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            })
            .insert(TopDownCamera::new(20.0))
            .insert_bundle(PickingCameraBundle::default())
            .insert(RayCastSource::<MyRayCastSet>::new());
    }

    fn start_playing_state(mut commands: Commands) {
        info!("Playing");
        commands.insert_resource(NextState(GameState::Playing));
    }

    fn start_spawn_player_state(mut commands: Commands) {
        info!("Spawn Player");
        commands.insert_resource(NextState(GameState::SpawnPlayer));
    }

    fn start_post_processing_state(mut commands: Commands) {
        info!("Post-processing");
        commands.insert_resource(NextState(GameState::PostProcessing));
    }

    fn teardown_entities(mut commands: Commands, entities: Query<Entity>) {
        info!("Teardown entities");
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn setup_scene_dynamic(
        colony_assets: Res<Assets<ColonyAsset>>,
        scene_assets: Res<SceneAssets>,
        asset_server: Res<AssetServer>,
        mut scene_spawner: ResMut<SceneSpawner>,
        mut game: ResMut<Game>,
    ) {
        info!("Setup scene");
        let current_colony = colony_assets.get(&game.current_colony_asset).unwrap();
        let colony_scene = match current_colony.name {
            Colony::Cloning => scene_assets.cloning.clone(),
            Colony::Iris => scene_assets.iris.clone(),
            Colony::Liberte => scene_assets.liberte.clone(),
            _ => scene_assets.liberte.clone(),
        };
        game.scene_handle = colony_scene.clone();
        scene_spawner.spawn_dynamic(colony_scene);
        asset_server.watch_for_changes().unwrap();
    }

    fn scene_loaded(query: Query<Entity, With<VortexNode>>) -> bool {
        if query.iter().count() > 0 {
            true
        } else {
            false
        }
    }
}

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VortexNode>();
        app.register_type::<VortexGate>();
        app.register_type::<BarrierField>();
        app.register_type::<BarrierAccess>();
        app.add_plugin(RonAssetPlugin::<ColonyAsset>::new(&["colony"]));
        app.add_plugin(PhysicsPlugin::default());
        app.add_plugin(VortexPlugin);
        app.add_plugins(DefaultPickingPlugins);
        app.add_plugin(BarrierPlugin);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::LoadColony)
                .with_system(Self::setup_scene_dynamic)
                .with_system(Self::setup_debug_plane)
                .with_system(Self::setup_zones)
                .into(),
        );
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::LoadColony)
                .run_if(Self::scene_loaded)
                .with_system(Self::start_post_processing_state)
                .into(),
        );
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::PostProcessing)
                .label(WorldSystem::SetupInsert)
                .with_system(Self::barrier_access_insert)
                .with_system(Self::vortex_gate_insert)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::PostProcessing)
                .after(WorldSystem::SetupInsert)
                .with_system(Self::start_spawn_player_state)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::SpawnPlayer)
                .label(WorldSystem::CameraSetup)
                .after(WorldSystem::PlayerSetup)
                .with_system(Self::setup_camera)
                .into(),
        );
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::SpawnPlayer)
                .after(WorldSystem::CameraSetup)
                .with_system(Self::start_playing_state)
                .into(),
        );
        app.add_exit_system(GameState::Playing, Self::teardown_entities);
    }
}
