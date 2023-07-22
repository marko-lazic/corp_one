use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins((DefaultPlugins, MaterialPlugin::<BarrierMaterial>::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut barrier_materials: ResMut<Assets<BarrierMaterial>>,
) {
    // sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })),
        material: barrier_materials.add(BarrierMaterial {
            color: Color::BLUE,
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 2.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "bac1a40f-4db1-4be7-8a11-b9ee3735c5f2"]
struct BarrierMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}

impl Material for BarrierMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/barrier.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
