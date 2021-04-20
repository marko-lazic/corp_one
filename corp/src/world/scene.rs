use crate::loading::MeshAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct ScenePlugin;

struct Cube;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()));
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(cube_movement.system()),
        );
    }
}

fn cube_movement(mut cube_positions: Query<(&Cube, &mut Transform)>) {
    for (_cube, mut transform) in cube_positions.iter_mut() {
        transform.translation.y += 0.01;
    }
}

fn setup(
    mut commands: Commands,
    mesh_assets: Res<MeshAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        }),
        ..Default::default()
    });
    // node mesh
    let node_mesh = mesh_assets.energy_node.clone();
    let cube_handle = mesh_assets.cube.clone();
    let cloned_node_mesh = node_mesh.clone();
    let green_material = materials.add(Color::rgb(0.1, 0.2, 0.1).into());
    let blue_material = materials.add(Color::rgb(0.1, 0.4, 0.8).into());
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: node_mesh,
        material: green_material.clone(),
        transform: Transform::from_translation(Vec3::new(-1.5, 1.0, 0.0)),
        ..Default::default()
    });
    // node mesh
    commands.spawn_bundle(PbrBundle {
        mesh: cloned_node_mesh,
        material: blue_material.clone(),
        transform: Transform::from_translation(Vec3::new(1.5, 1.0, 0.0)),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle.clone(),
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(10.0, 1.0, -10.0)),
            ..Default::default()
        })
        .insert(Cube);
}
