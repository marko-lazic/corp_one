use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCamera, PickingCameraBundle, PickingPlugin};
use bevy_mod_raycast::Ray3d;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::build()
        .init_resource::<Game>()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_plugin(PickingPlugin)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_look_at_hit_point.system())
                .with_system(player_move.system()),
        )
        .run();
}

fn player_move(
    mut query: Query<(&mut Transform, &PlayerSpeed)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut transform, speed)) = query.single_mut() {
        let mut dir_x = 0.0;
        let mut dir_z = 0.0;
        if keyboard_input.pressed(KeyCode::A) {
            dir_x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            dir_x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            dir_z -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            dir_z += 1.0;
        }

        transform.translation.x += dir_x * speed.speed * TIME_STEP;
        transform.translation.z += dir_z * speed.speed * TIME_STEP;
    }
}

fn player_look_at_hit_point(
    mut lines: ResMut<DebugLines>,
    pickings: Query<&PickingCamera>,
    mut query: Query<(&mut Transform, &PlayerSpeed)>,
) {
    for raycast_source in pickings.iter() {
        match raycast_source.intersect_top() {
            Some(top_intersection) => {
                let transform_new = top_intersection.1.normal_ray().to_transform();
                let mouse_world = Transform::from_matrix(transform_new);
                if let Ok((mut transform, _)) = query.single_mut() {
                    let hit_point = Ray3d::new(transform.translation, mouse_world.translation);
                    let aim_point =
                        Vec3::new(hit_point.direction().x, 0.5, hit_point.direction().z);
                    transform.look_at(aim_point, Vec3::Y);
                    lines.line(transform.translation, mouse_world.translation, 0.);
                }
            }
            None => {}
        }
    }
}

fn setup(
    mut game: ResMut<Game>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
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

    game.player.transform = Transform::from_xyz(0.0, 0.5, 0.0);
    // player
    game.player.entity = Some(
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: game.player.transform,
                ..Default::default()
            })
            .insert(PlayerSpeed::new(2.))
            .id(),
    );
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    transform: Transform,
}

struct PlayerSpeed {
    speed: f32,
}

impl PlayerSpeed {
    fn new(speed: f32) -> Self {
        PlayerSpeed { speed }
    }
}

#[derive(Default)]
struct Game {
    player: Player,
}
