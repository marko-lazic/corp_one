use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_mod_picking::RayCastSource;
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
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::vortex::{VortexGate, VortexNode, VortexPlugin};
use crate::world::colony::zone::{Zone, ZoneEntities};
use crate::world::player::PlayerBundle;
use crate::Game;

mod asset;
pub mod colony_assets;
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
    Setup,
    Enrich,
    Player,
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

    fn vortex_gate_insert(mut commands: Commands, query: Query<Entity, With<VortexGate>>) {
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

    fn setup_player(
        mut commands: Commands,
        mesh_assets: Res<MeshAssets>,
        materials: ResMut<Assets<StandardMaterial>>,
        mut game: ResMut<Game>,
        mut vortex_nodes: Query<&mut Transform, With<VortexNode>>,
    ) {
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
            .insert(RayCastSource::<MyRayCastSet>::new());
    }

    fn start_playing_state(mut game_state: ResMut<State<GameState>>) {
        info!("Start playing state");
        let _ = game_state.set(GameState::Playing);
    }

    fn teardown_entities(mut commands: Commands, entities: Query<Entity>) {
        info!("Teardown entities");
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn setup_scene_sync(world: &mut World) {
        info!("Setup scene");
        let game = world.get_resource::<Game>().unwrap();
        let colony_assets = world.get_resource::<Assets<ColonyAsset>>().unwrap();
        let scene_assets = world.get_resource::<SceneAssets>().unwrap();
        let current_colony = colony_assets.get(&game.current_colony_asset).unwrap();
        let colony_scene = match current_colony.name {
            Colony::Cloning => scene_assets.cloning.clone(),
            Colony::Iris => scene_assets.iris.clone(),
            Colony::Liberte => scene_assets.liberte.clone(),
            _ => scene_assets.liberte.clone(),
        };
        let mut spawner = SceneSpawner::from_world(world);
        let _ = spawner.spawn_dynamic_sync(world, &colony_scene);
    }

    // fn setup_scene_dynamic(scene_assets: Res<SceneAssets>, mut spawner: ResMut<SceneSpawner>) {
    //     info!("Setup environment");
    //     spawner.spawn_dynamic(scene_assets.cloning.clone());
    // }
}

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VortexNode>();
        app.register_type::<VortexGate>();
        app.add_plugin(RonAssetPlugin::<ColonyAsset>::new(&["colony"]));
        app.add_plugin(PhysicsPlugin::default());
        app.add_plugin(VortexPlugin);
        app.add_system_set(
            SystemSet::on_enter(GameState::LoadColony)
                .label(ColonySystem::Setup)
                .with_system(Self::setup_scene_sync.exclusive_system())
                .with_system(Self::setup_debug_plane.system())
                .with_system(Self::setup_zones.system()),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::LoadColony)
                .after(ColonySystem::Setup)
                .label(ColonySystem::Enrich)
                .with_system(Self::vortex_gate_insert.system()),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::LoadColony)
                .after(ColonySystem::Setup)
                .label(ColonySystem::Player)
                .with_system(Self::setup_player.system())
                .with_system(Self::setup_camera.system()),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::LoadColony)
                .after(ColonySystem::Player)
                .after(ColonySystem::Enrich)
                .with_system(Self::start_playing_state),
        );

        app.add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(Self::teardown_entities.system()),
        );
    }
}
