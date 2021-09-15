use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_mod_bounding::{aabb, debug, Bounded};
use bevy_mod_raycast::RayCastMesh;

use crate::asset::asset_loading::{MaterialAsset, MaterialAssets, MeshAssets};
use crate::constants::state::GameState;
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::vortex::VortexPlugin;
use crate::world::colony::zone::{Zone, ZoneType};
use crate::world::cursor::MyRaycastSet;
use crate::Game;

mod asset;
pub mod colony_assets;
mod vortex;
pub mod zone;

pub struct ColonyPlugin;

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
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(RayCastMesh::<MyRaycastSet>::default());
    }

    fn setup_light(mut commands: Commands, game: Res<Game>, assets: Res<Assets<ColonyAsset>>) {
        if let Some(colony_asset) = assets.get(&game.current_colony_asset) {
            for light in &colony_asset.lights {
                commands.spawn_bundle(LightBundle {
                    light: Light {
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
                    .insert(RayCastMesh::<MyRaycastSet>::default());
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
            for zone in &colony_asset.zones {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane {
                            size: zone.size.clone(),
                        })),
                        transform: Transform::from_translation(zone.position),
                        material: material_assets.get_material(&zone.material),
                        ..Default::default()
                    })
                    .insert(Bounded::<aabb::Aabb>::default())
                    .insert(debug::DebugBounds)
                    .insert(Zone::new(zone.zone_type));
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
                    .insert(Bounded::<aabb::Aabb>::default())
                    .insert(debug::DebugBounds)
                    .insert(Zone::new(ZoneType::VortexGate));
            }
        }
    }

    fn cleanup_colony(mut commands: Commands, entities: Query<Entity>, mut game: ResMut<Game>) {
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        game.is_vorting = false;
    }
}

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RonAssetPlugin::<ColonyAsset>::new(&["colony"]));
        app.add_plugin(VortexPlugin);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(Self::setup_plane.system())
                .with_system(Self::setup_light.system())
                .with_system(Self::setup_energy_nodes.system())
                .with_system(Self::setup_zones.system())
                .with_system(Self::setup_vortex_gates.system()),
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(Self::cleanup_colony.system()),
        );
    }
}
