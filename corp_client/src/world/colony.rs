use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_mod_raycast::RayCastMesh;

use crate::asset::asset_loading::ColonyAssets;
use crate::constants::state::GameState;
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::cursor::MyRaycastSet;

pub mod colony_assets;
mod vortex;
mod asset_color;

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
}

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RonAssetPlugin::<ColonyAsset>::new(&["colony"]));
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(Self::setup_plane.system())
                .with_system(Self::setup_light.system()),
        );
    }
}