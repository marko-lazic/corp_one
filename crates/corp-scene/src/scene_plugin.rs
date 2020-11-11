pub mod scene {
    use crate::player::Player;
    use bevy::prelude::*;

    pub struct ScenePlugin;

    impl Plugin for ScenePlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_startup_system(setup.system());
        }
    }

    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // add entities to the world
        // Plane
        commands.spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)),
            material: materials.add(StandardMaterial {
                albedo: Color::WHITE,
                ..Default::default()
            }),
            ..Default::default()
        });
        // node mesh
        let node_mesh = asset_server.load("models/node/node_template.gltf#Mesh0/Primitive0");
        let cloned_node_mesh = node_mesh.clone();
        let node_material = materials.add(Color::rgb(0.1, 0.2, 0.1).into());
        let node_material_2 = materials.add(Color::rgb(0.1, 0.4, 0.8).into());
        let material_handle = materials.add(StandardMaterial {
            albedo: Color::rgb(0.8, 0.7, 0.6),
            ..Default::default()
        });
        commands
            .spawn(PbrComponents {
                mesh: node_mesh,
                material: node_material.clone(),
                transform: Transform::from_translation(Vec3::new(-1.5, 0.0, 0.0)),
                ..Default::default()
            }) // mesh
            // node mesh
            .spawn(PbrComponents {
                mesh: cloned_node_mesh,
                material: node_material_2.clone(),
                transform: Transform::from_translation(Vec3::new(1.5, 0.0, 0.0)),
                ..Default::default()
            })
            // light
            .spawn(LightComponents {
                transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
                ..Default::default()
            });

        // camera
        let camera_entity = commands
            .spawn(Camera3dComponents {
                transform: Transform::from_translation(Vec3::new(-5.0, 5.0, 10.0))
                    .looking_at(Vec3::default(), Vec3::unit_y()),
                ..Default::default()
            })
            .current_entity();

        // Player ball
        let player_entity = commands
            .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.0,
                    subdivisions: 3,
                })),
                material: materials.add(StandardMaterial {
                    albedo: Color::GREEN,
                    ..Default::default()
                }),
                transform: Transform::from_matrix(Mat4::identity()),
                ..Default::default()
            })
            .with(Player {
                camera_entity,
                ..Default::default()
            })
            .current_entity();

        commands
            // Append camera to player as child.
            .push_children(player_entity.unwrap(), &[camera_entity.unwrap()]);

        let cube_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
        commands.spawn(PbrComponents {
            mesh: cube_handle,
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(10.0, 1.0, -10.0)),
            ..Default::default()
        });
    }
}
