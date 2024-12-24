use bevy::prelude::*;
use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};

fn main() {
    App::new()
        .init_resource::<InspectorData>()
        .init_resource::<GameData>()
        .init_gizmo_group::<MyRoundGizmos>()
        .add_plugins((
            DefaultPlugins,
            ResourceInspectorPlugin::<InspectorData>::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_distance_vec)
        .run();
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

#[derive(Component)]
struct Movable;

#[derive(Resource, Default)]
struct GameData {
    sphere: Option<Entity>,
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct InspectorData {
    #[inspector(min = 0.0, max = 100.0)]
    offset: f32,
}

fn draw_distance_vec(
    mut gizmos: Gizmos<MyRoundGizmos>,
    game: Res<GameData>,
    mut query: Query<&mut Transform, With<Movable>>,
    inspector: Res<InspectorData>,
) {
    let zero = Vec3::ZERO;
    let a = Vec3::new(3., 2., 0.);
    let b = Vec3::new(-5., 3., 0.);
    let red = Color::LinearRgba(LinearRgba::RED);
    let green = Color::LinearRgba(LinearRgba::GREEN);
    let blue = Color::LinearRgba(LinearRgba::BLUE);

    let a_to_b = b - a; // to point (b) minus (-) from point (a)
    let a_to_b_dir = a_to_b.normalize();
    let c = a + a_to_b_dir * 8.0;
    gizmos.line(zero, a, red);
    gizmos.line(zero, b, blue);
    gizmos.line(a, c, green);

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
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)),
    ));

    // sphere
    let sphere_id = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.1).mesh().ico(7).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::LinearRgba(LinearRgba::GREEN),
                ..Default::default()
            })),
            Transform::from_xyz(1.5, 1.0, 1.5),
            Movable,
        ))
        .id();

    game.sphere = Some(sphere_id);
}
