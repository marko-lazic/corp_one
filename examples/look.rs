use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_mod_raycast::Ray3d;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup.system())
        .add_system_to_stage(CoreStage::PostUpdate, rotate_object.system())
        .run();
}

pub fn rotate_object(
    windows: Res<Windows>,
    mut cursor: EventReader<CursorMoved>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut seeker_query: Query<&mut Transform, With<Seeker>>,
    mut lines: ResMut<DebugLines>,
) {
    for (camera_transform, camera) in camera_query.iter() {
        let cursor_latest = match cursor.iter().last() {
            Some(cursor_moved) => {
                if cursor_moved.id == camera.window {
                    Some(cursor_moved)
                } else {
                    None
                }
            }
            None => None,
        };
        match cursor_latest {
            Some(cursor_moved) => {
                let ray_option = Ray3d::from_screenspace(
                    cursor_moved.position,
                    &windows,
                    camera,
                    camera_transform,
                );

                match ray_option {
                    Some(ray) => {
                        lines.line(ray.origin(), ray.direction(), 0.);
                        for mut transform in seeker_query.iter_mut() {
                            transform.look_at(ray.direction(), Vec3::Y);
                            transform.rotation = Quat::from_rotation_y(transform.rotation.y);
                        }
                    }
                    None => println!("no ray"),
                }
            }
            None => {}
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    // seeker
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Seeker);
}

pub struct Seeker;
