pub mod scene {
    use bevy::app::{AppBuilder, Plugin};
    use bevy::asset::{AssetServer, Assets};
    use bevy::ecs::{Commands, IntoQuerySystem, Res, ResMut};
    use bevy::math::{FaceToward, Mat4, Vec3};
    use bevy::pbr::{LightComponents, PbrComponents};
    use bevy::prelude::{StandardMaterial, Handle};
    use bevy::render::color::Color;
    use bevy::render::entity::Camera3dComponents;
    use bevy::transform::components::Transform;
    use bevy::render::mesh::Mesh;

    pub struct ScenePlugin;

    impl Plugin for ScenePlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_startup_system(setup.system());
        }
    }

    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mesh = load_node(asset_server);
        // add entities to the world
        commands
            // node mesh
            .spawn(PbrComponents {
                mesh,
                material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
                transform: Transform::from_translation(Vec3::new(-1.5, 0.0, 0.0)),
                ..Default::default()
            })// mesh
            // node mesh
            .spawn(PbrComponents {
                mesh,
                material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
                transform: Transform::from_translation(Vec3::new(1.5, 0.0, 0.0)),
                ..Default::default()
            })
            // light
            .spawn(LightComponents {
                transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
                ..Default::default()
            })
            // camera
            .spawn(Camera3dComponents {
                transform: Transform::new(Mat4::face_toward(
                    Vec3::new(-2.0, 2.0, 6.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                )),
                ..Default::default()
            });
    }

    fn load_node(asset_server: Res<AssetServer>) -> Handle<Mesh> {
        asset_server
            .load("assets/models/node/node_template.gltf")
            .unwrap()
    }
}
