use bevy::prelude::*;
use heron::prelude::*;

use bevy_asset_ron::RonAssetPlugin;
use bevy_mod_picking::RayCastSource;
use bevy_mod_raycast::RayCastMesh;

use corp_shared::prelude::Player;

use crate::asset::asset_loading::{MaterialAsset, MaterialAssets, MeshAssets};
use crate::constants::state::GameState;
use crate::input::MyRayCastSet;
use crate::world::camera::{CameraCenter, TopDownCamera};
use crate::world::character::Movement;
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::vortex::{VortexNode, VortexPlugin};
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

#[derive(Debug)]
pub enum Colony {
    StarMap,
    Cloning,
    Iris,
    Liberte,
}

impl Default for Colony {
    fn default() -> Self {
        Self::StarMap
    }
}

pub struct ColonyPlugin;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum ColonySystem {
    Environment,
}

impl ColonyPlugin {
    fn setup_plane(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
                transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
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

    fn setup_light(mut commands: Commands, game: Res<Game>, assets: Res<Assets<ColonyAsset>>) {
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            for light in &colony_asset.lights {
                commands.spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        color: light.color.get_color().clone(),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(light.position),
                    ..Default::default()
                });
            }
        }
    }

    fn setup_energy_nodes(
        mut commands: Commands,
        game: Res<Game>,
        assets: Res<Assets<ColonyAsset>>,
        material_assets: Res<MaterialAssets>,
        mesh_assets: Res<MeshAssets>,
    ) {
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            for energy_node in &colony_asset.energy_nodes {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: mesh_assets.energy_node.clone(),
                        material: material_assets.get_material(&energy_node.material),
                        transform: Transform::from_translation(energy_node.position),
                        ..Default::default()
                    })
                    .insert(RayCastMesh::<MyRayCastSet>::default());
            }
        }
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

    fn setup_vortex_gates(
        mut commands: Commands,
        game: Res<Game>,
        assets: Res<Assets<ColonyAsset>>,
        material_assets: Res<MaterialAssets>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            for vortex_gate in &colony_asset.vortex_gates {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                        transform: Transform::from_translation(vortex_gate.position),
                        material: material_assets.get_material(&MaterialAsset::SkyBlue),
                        ..Default::default()
                    })
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
    }

    fn setup_vortex_nodes(
        mut commands: Commands,
        game: Res<Game>,
        assets: Res<Assets<ColonyAsset>>,
        mesh_assets: Res<MeshAssets>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            for vortex_node in &colony_asset.vortex_nodes {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: mesh_assets.vortex_node.clone(),
                        transform: Transform {
                            scale: Vec3::new(0.3, 0.3, 0.3),
                            translation: Vec3::new(
                                vortex_node.position.x.clone(),
                                3.0,
                                vortex_node.position.z.clone(),
                            ),
                            ..Default::default()
                        },
                        material: {
                            let material = materials.add(Color::rgba(1.0, 0.9, 0.9, 0.4).into());
                            material
                        },
                        visibility: Visibility { is_visible: true },
                        ..Default::default()
                    })
                    .insert(VortexNode);
            }
        }
    }

    fn setup_player(
        mut commands: Commands,
        mesh_assets: Res<MeshAssets>,
        materials: ResMut<Assets<StandardMaterial>>,
        mut game: ResMut<Game>,
        assets: Res<Assets<ColonyAsset>>,
    ) {
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            let position = colony_asset
                .random_vortex_node_position()
                .unwrap_or_default();

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

    fn teardown(mut commands: Commands, entities: Query<Entity>) {
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<ColonyAsset>::new(&["colony"]));
        app.add_plugin(PhysicsPlugin::default());
        app.add_plugin(VortexPlugin);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .label(ColonySystem::Environment)
                .with_system(Self::setup_plane.system())
                .with_system(Self::setup_light.system())
                .with_system(Self::setup_energy_nodes.system())
                .with_system(Self::setup_zones.system())
                .with_system(Self::setup_vortex_gates.system())
                .with_system(Self::setup_vortex_nodes.system()),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .after(ColonySystem::Environment)
                .with_system(Self::setup_player.system())
                .with_system(Self::setup_camera.system()),
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(Self::teardown.system()),
        );
    }
}
