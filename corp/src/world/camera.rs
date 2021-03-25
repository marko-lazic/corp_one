use bevy::ecs::{Commands, Entity};
use bevy::math::{Mat4, Quat, Vec3};
use bevy::prelude::Camera3dBundle;
use bevy::transform::components::Transform;

pub fn spawn_camera(commands: &mut Commands) -> Option<Entity> {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .current_entity()
}
