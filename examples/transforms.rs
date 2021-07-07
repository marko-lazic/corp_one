use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotate_cubes.system())
        .add_system(rotate_spheres.system())
        .run();
}

/// set up a simple 3D scene
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
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(CubeRotator);

    // sphere
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.45,
                subdivisions: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("ffd891").unwrap(),
                ..Default::default()
            }),
            transform: Transform::from_xyz(3.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(SphereRotator)
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.10,
                    subdivisions: 6,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.7, 0.0, 0.0),
                ..Default::default()
            });
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
}

fn rotate_cubes(time: Res<Time>, mut cubes: Query<&mut Transform, With<CubeRotator>>) {
    for mut transform in cubes.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(3.0 * time.delta_seconds());
    }
}

fn rotate_spheres(time: Res<Time>, mut spheres: Query<&mut Transform, With<SphereRotator>>) {
    for mut transform in spheres.iter_mut() {
        transform.look_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y);
        transform.translation = transform.mul_vec3(Vec3::new(-0.1, 0.0, 0.0));
        transform.rotate(Quat::from_rotation_y(3.0 * time.delta_seconds()));
    }
}

struct CubeRotator;

struct SphereRotator;
