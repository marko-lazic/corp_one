use bevy::prelude::*;
pub struct ScenePlugin;

struct Cube;

struct Materials {
    cube_mesh: Handle<Mesh>,
    cube_material: Handle<StandardMaterial>,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_startup_stage("game_setup", SystemStage::single(spawn_cube.system()));
        app.add_system(cube_movement.system());
    }
}

fn cube_movement(mut cube_positions: Query<(&Cube, &mut Transform)>) {
    for (_cube, mut transform) in cube_positions.iter_mut() {
        transform.translation.y += 0.01;
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        material: materials.add(StandardMaterial {
            albedo: Color::WHITE,
            ..Default::default()
        }),
        ..Default::default()
    });
    // node mesh
    let node_mesh = asset_server.load("models/node/node_template.gltf#Mesh0/Primitive0");
    let cloned_node_mesh = node_mesh.clone();
    let green_material = materials.add(Color::rgb(0.1, 0.2, 0.1).into());
    let blue_material = materials.add(Color::rgb(0.1, 0.4, 0.8).into());
    let material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    commands
        .spawn(PbrBundle {
            mesh: node_mesh,
            material: green_material.clone(),
            transform: Transform::from_translation(Vec3::new(-1.5, 1.0, 0.0)),
            ..Default::default()
        }) // mesh
        // node mesh
        .spawn(PbrBundle {
            mesh: cloned_node_mesh,
            material: blue_material.clone(),
            transform: Transform::from_translation(Vec3::new(1.5, 1.0, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    let cube_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
    commands.insert_resource(Materials {
        cube_mesh: cube_handle,
        cube_material: material_handle.clone(),
    });
}

fn spawn_cube(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(PbrBundle {
            mesh: materials.cube_mesh.clone(),
            material: materials.cube_material.clone(),
            transform: Transform::from_translation(Vec3::new(10.0, 1.0, -10.0)),
            ..Default::default()
        })
        .with(Cube);
}
