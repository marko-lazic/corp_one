use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, RayCastSource};
use bevy_mod_raycast::RayCastMesh;
use heron::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;

use corp_shared::prelude::Player;

use crate::asset::asset_loading::{MaterialAssets, MeshAssets, SceneAssets};
use crate::constants::state::GameState;
use crate::input::MyRayCastSet;
use crate::world::camera::{CameraCenter, TopDownCamera};
use crate::world::character::Movement;
use crate::world::colony::barrier::{BarrierAccess, BarrierField, BarrierPlugin};
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::vortex::{VortexGate, VortexNode, VortexPlugin};
use crate::world::colony::zone::{Zone, ZoneEntities};
use crate::world::player::PlayerBundle;
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

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum ColonySystem {
    PlayerSetup,
    CameraSetup,
    BarrierInsert,
    VortexGateInsert,
}

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

    fn setup_player(
        mut commands: Commands,
        mesh_assets: Res<MeshAssets>,
        materials: ResMut<Assets<StandardMaterial>>,
        mut game: ResMut<Game>,
        mut vortex_nodes: Query<&mut Transform, With<VortexNode>>,
        asset_server: Res<AssetServer>,
    ) {
        asset_server.watch_for_changes().unwrap();
        let random_position = vortex_nodes
            .iter_mut()
            .map(|t| t.translation)
            .collect::<Vec<Vec3>>()
            .choose(&mut rand::thread_rng())
            .map(|p| p.to_owned());

        let position = random_position.unwrap_or_else(|| Vec3::new(1.0, 1.0, 1.0));

        let player = commands
            .spawn_bundle(PlayerBundle::new(mesh_assets, materials, position))
            .insert(Player::default())
            .insert(Movement::default())
            .insert(game.health.clone())
            .insert(CameraCenter)
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(0.5, 1.0, 0.5),
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Player)
                    .with_masks(vec![Layer::Zone, Layer::VortexGate]),
            )
            .id();

        game.player_entity = Some(player);
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

    fn start_playing_state(mut game_state: ResMut<State<GameState>>) {
        info!("Playing");
        let _ = game_state.set(GameState::Playing);
    }

    fn start_spawn_player_state(mut game_state: ResMut<State<GameState>>) {
        info!("Spawn Player");
        let _ = game_state.set(GameState::SpawnPlayer);
    }

    fn start_post_processing_state(mut game_state: ResMut<State<GameState>>) {
        info!("Post-processing");
        let _ = game_state.set(GameState::PostProcessing);
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

    fn scene_loaded(
        game_state: Res<State<GameState>>,
        query: Query<Entity, With<VortexNode>>,
    ) -> ShouldRun {
        if query.iter().count() > 0 && game_state.current() == &GameState::LoadColony {
            ShouldRun::Yes
        } else {
            ShouldRun::No
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
            SystemSet::on_enter(GameState::LoadColony)
                .with_system(Self::setup_scene_dynamic)
                .with_system(Self::setup_debug_plane)
                .with_system(Self::setup_zones),
        );
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(Self::scene_loaded)
                .with_system(Self::start_post_processing_state),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::PostProcessing)
                .with_system(Self::barrier_access_insert.label(ColonySystem::BarrierInsert))
                .with_system(Self::vortex_gate_insert.label(ColonySystem::VortexGateInsert))
                .with_system(
                    Self::start_spawn_player_state
                        .after(ColonySystem::VortexGateInsert)
                        .after(ColonySystem::BarrierInsert),
                ),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::SpawnPlayer)
                .with_system(Self::setup_player.label(ColonySystem::PlayerSetup))
                .with_system(
                    Self::setup_camera
                        .label(ColonySystem::CameraSetup)
                        .after(ColonySystem::PlayerSetup),
                ),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::SpawnPlayer).with_system(
                Self::start_playing_state
                    .after(ColonySystem::PlayerSetup)
                    .after(ColonySystem::CameraSetup),
            ),
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(Self::teardown_entities),
        );
    }
}
