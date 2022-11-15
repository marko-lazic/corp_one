use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<GameData>()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<InspectorData>::new())
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup)
        .add_system(draw_distance_vec)
        .run();
}

fn draw_distance_vec(
    mut lines: ResMut<DebugLines>,
    game: Res<GameData>,
    mut query: Query<&mut Transform, With<Movable>>,
    inspector: Res<InspectorData>,
) {
    let zero = Vec3::ZERO;
    let a = Vec3::new(3., 2., 0.);
    let b = Vec3::new(-5., 3., 0.);
    let red = Color::RED;
    let green = Color::GREEN;
    let blue = Color::BLUE;

    let a_to_b = b - a; // to point (b) minus (-) from point (a)
    let a_to_b_dir = a_to_b.normalize();
    let c = a + a_to_b_dir * 8.0;
    lines.line_colored(zero, a, 0.0, red);
    lines.line_colored(zero, b, 0.0, blue);
    lines.line_colored(a, c, 0.0, green);

    // let midpoint = (a - b) / 2.;
    let mut sphere = query.get_mut(game.sphere.unwrap()).unwrap();
    let offset_vec = a_to_b_dir * inspector.offset;
    sphere.translation = a + offset_vec;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<GameData>,
) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)),
        ..Default::default()
    });

    // sphere
    let sphere_id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..Default::default()
            }),
            transform: Transform::from_xyz(1.5, 1.0, 1.5),
            ..Default::default()
        })
        .insert(Movable)
        .id();

    game.sphere = Some(sphere_id);
}

#[derive(Component)]
struct Movable;

#[derive(Default)]
struct GameData {
    sphere: Option<Entity>,
}

#[derive(Inspectable, Default)]
struct InspectorData {
    #[inspectable(min = 0.0, max = 100.0)]
    offset: f32,
}
