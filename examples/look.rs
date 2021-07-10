use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCamera, PickingCameraBundle, PickingPlugin};
use bevy_mod_raycast::Ray3d;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_plugin(PickingPlugin)
        .add_startup_system(setup.system())
        .add_system(rotate_object.system())
        .run();
}

fn rotate_object(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut lines: ResMut<DebugLines>,
    picking_camera_query: Query<&PickingCamera>,
) {
    for mut player_transform in player_transform_query.iter_mut() {
        for raycast_source in picking_camera_query.iter() {
            match raycast_source.intersect_top() {
                Some(top_intersection) => {
                    let transform_new = top_intersection.1.normal_ray().to_transform();
                    let mouse_world = Transform::from_matrix(transform_new);
                    let hit_point =
                        Ray3d::new(player_transform.translation, mouse_world.translation);
                    let aim_point =
                        Vec3::new(hit_point.direction().x, 0.5, hit_point.direction().z);
                    player_transform.look_at(aim_point, Vec3::Y);
                    lines.line(hit_point.origin(), hit_point.direction(), 0.);
                }
                None => {}
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // Make this mesh ray cast-able
        .insert_bundle(PickableBundle::default());

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        // Designate the camera as our source
        .insert_bundle(PickingCameraBundle::default());

    // seeker
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Player);
}

struct Player;
