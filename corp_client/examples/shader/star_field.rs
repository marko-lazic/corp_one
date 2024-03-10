use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (640.0, 480.0).into(),
                    ..default()
                }),
                ..default()
            }),
            MaterialPlugin::<StarFieldMaterial>::default(),
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, update_star_field))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<StarFieldMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(4.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
        MainCamera,
    ));
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::default()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: custom_materials.add(StarFieldMaterial {
            mouse: Vec2::new(0.0, 0.0),
            speed2: 0.2,
        }),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: standard_materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
}

fn rotate_camera(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        let cam_transform = camera.single_mut().into_inner();

        cam_transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Y, 45f32.to_radians() * time.delta_seconds()),
        );
        cam_transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn update_star_field(
    material_handle: Query<&Handle<StarFieldMaterial>>,
    mut materials: ResMut<Assets<StarFieldMaterial>>,
    primary_query: Query<&Window>,
) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };

    let handle = material_handle.single();
    let mat = materials.get_mut(handle).unwrap();

    if let Some(position) = primary.cursor_position() {
        mat.mouse.x = position.x;
        mat.mouse.y = position.y;
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarFieldMaterial {
    #[uniform(0)]
    mouse: Vec2,
    #[uniform(0)]
    speed2: f32,
}

impl Material for StarFieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders_ex/star_field_ex.wgsl".into()
    }
}
