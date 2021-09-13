use bevy::prelude::*;
use bevy_mod_bounding::{aabb, debug, Bounded};
use bevy_mod_raycast::RayCastMesh;

use crate::asset::asset_loading::MeshAssets;
use crate::constants::state::GameState;
use crate::world::cursor::MyRaycastSet;
use crate::world::flying_cubes::FlyingCubesPlugin;
use crate::world::zone::{Zone, ZoneType};

pub struct ScenePlugin;

impl ScenePlugin {
    fn setup(
        mut commands: Commands,
        mesh_assets: Res<MeshAssets>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // Add entities to the world
        // Plane
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

        // Node mesh
        let node_mesh = mesh_assets.energy_node.clone();
        let cloned_node_mesh = node_mesh.clone();
        let green_material = materials.add(Color::rgb(0.1, 0.2, 0.1).into());
        let blue_material = materials.add(Color::rgb(0.1, 0.4, 0.8).into());

        // Energy nodes
        commands
            .spawn_bundle(PbrBundle {
                mesh: node_mesh,
                material: green_material.clone(),
                transform: Transform::from_translation(Vec3::new(-1.5, 1.0, 0.0)),
                ..Default::default()
            })
            .insert(RayCastMesh::<MyRaycastSet>::default());

        commands
            .spawn_bundle(PbrBundle {
                mesh: cloned_node_mesh,
                material: blue_material.clone(),
                transform: Transform::from_translation(Vec3::new(1.5, 1.0, 0.0)),
                ..Default::default()
            })
            .insert(RayCastMesh::<MyRaycastSet>::default());
        // Damage zone
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                transform: Transform::from_translation(Vec3::new(-3., 0.1, -4.)),
                material: materials.add(StandardMaterial {
                    base_color: Color::ORANGE_RED,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(Zone::new(ZoneType::Damage(3.3)))
            .insert(Bounded::<aabb::Aabb>::default())
            .insert(debug::DebugBounds);
        // Heal zone
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                transform: Transform::from_translation(Vec3::new(3., 0.1, -4.)),
                material: materials.add(StandardMaterial {
                    base_color: Color::SEA_GREEN,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(Zone::new(ZoneType::Heal(0.5)))
            .insert(Bounded::<aabb::Aabb>::default())
            .insert(debug::DebugBounds);

        // Light
        commands.spawn_bundle(LightBundle {
            light: Light {
                color: Color::LIME_GREEN,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
        commands.spawn_bundle(LightBundle {
            light: Light {
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-8.0, 8.0, -8.0)),
            ..Default::default()
        });
    }
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(Self::setup.system()),
        );
        app.add_plugin(FlyingCubesPlugin);
    }
}
