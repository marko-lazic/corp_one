use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_mod_raycast::RayCastMesh;

use crate::asset::asset_loading::{ColonyAssets, MaterialAssets, MeshAssets};
use crate::constants::state::GameState;
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::cursor::MyRaycastSet;

mod asset;
pub mod colony_assets;
mod vortex;

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

    fn setup_light(
        mut commands: Commands,
        colony_assets: Res<ColonyAssets>,
        assets: Res<Assets<ColonyAsset>>,
    ) {
        if let Some(colony_asset) = assets.get(&colony_assets.iris) {
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
        colony_assets: Res<ColonyAssets>,
        assets: Res<Assets<ColonyAsset>>,
        material_assets: Res<MaterialAssets>,
        mesh_assets: Res<MeshAssets>,
    ) {
        if let Some(colony_asset) = assets.get(&colony_assets.iris) {
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
}

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RonAssetPlugin::<ColonyAsset>::new(&["colony"]));
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(Self::setup_plane.system())
                .with_system(Self::setup_light.system())
                .with_system(Self::setup_energy_nodes.system()),
        );
    }
}
