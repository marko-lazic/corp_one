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
            MaterialPlugin::<CustomMaterial>::default(),
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
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
        material: custom_materials.add(CustomMaterial {
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: standard_materials.add(Color::srgb(0.3, 0.5, 0.3)),
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders_ex/barrier_ex.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
