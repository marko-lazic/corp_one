use bevy::app::App;
use bevy::asset::Assets;
use bevy::pbr::{AlphaMode, Material, MaterialMeshBundle, MaterialPlugin};
use bevy::prelude::{shape, Camera3dBundle, ClearColor, Color, Commands, Mesh, ResMut, Transform};
use bevy::reflect::TypeUuid;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::utils::default;
use bevy::DefaultPlugins;
use glam::Vec3;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<BarrierMaterial>::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut barrier_materials: ResMut<Assets<BarrierMaterial>>,
) {
    // sphere
    commands.spawn().insert_bundle(MaterialMeshBundle {
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

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 2.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
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
