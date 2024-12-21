use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cubes, rotate_spheres))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..Default::default()
    });

    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(CubeRotator);

    // sphere
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(0.45).mesh().ico(32).unwrap()),
            material: materials.add(StandardMaterial {
                base_color: Color::Srgba(Srgba::hex("ffd891").unwrap()),
                ..Default::default()
            }),
            transform: Transform::from_xyz(3.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(SphereRotator)
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.10).mesh().ico(6).unwrap()),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.7, 0.0, 0.0),
                ..Default::default()
            });
        });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            intensity: 200.0,
            range: 20.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn rotate_cubes(time: Res<Time>, mut cubes: Query<&mut Transform, With<CubeRotator>>) {
    for mut transform in cubes.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(3.0 * time.delta_seconds());
    }
}

fn rotate_spheres(time: Res<Time>, mut spheres: Query<&mut Transform, With<SphereRotator>>) {
    for mut transform in spheres.iter_mut() {
        transform.look_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y);
        transform.translation = transform.transform_point(Vec3::new(-0.1, 0.0, 0.0));
        transform.rotate(Quat::from_rotation_y(3.0 * time.delta_seconds()));
    }
}

#[derive(Component)]
struct CubeRotator;

#[derive(Component)]
struct SphereRotator;
