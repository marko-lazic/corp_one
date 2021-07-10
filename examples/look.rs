use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, Ray3d, RayCastMesh, RayCastMethod, RayCastSource};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_plugin(DefaultRaycastingPlugin::<SeekRaycast>::default())
        .add_startup_system(setup.system())
        .add_system(rotate_object.system())
        .add_system(update_raycast_with_cursor.system())
        .run();
}

fn rotate_object(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut lines: ResMut<DebugLines>,
    seek_ray_query: Query<&RayCastSource<SeekRaycast>>,
) {
    for mut player_transform in player_transform_query.iter_mut() {
        for raycast_source in seek_ray_query.iter() {
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

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<SeekRaycast>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
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
        .insert(RayCastMesh::<SeekRaycast>::default());

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
        .insert(RayCastSource::<SeekRaycast>::new());

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

// Mark our generic `RayCastMesh`s and `RayCastSource`s as part of the same group, or "RayCastSet".
struct SeekRaycast;
