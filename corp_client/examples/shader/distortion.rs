use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
    },
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
            PanOrbitCameraPlugin,
            MaterialPlugin::<DistortionMaterial>::default(),
            // PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, update_distortion))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut custom_materials: ResMut<Assets<DistortionMaterial>>,
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
        material: custom_materials.add(DistortionMaterial {
            alpha_mode: AlphaMode::Blend,
            vertex_uniforms: Default::default(),
            fragment_uniforms: Default::default(),
            noise_view_x: Some(asset_server.load("shaders_ex/noise/noise_texture_1.png")),
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

#[allow(unused)] // The system should be able to change distortion parameters from the game
fn update_distortion(
    material_handle: Query<&Handle<DistortionMaterial>>,
    mut materials: ResMut<Assets<DistortionMaterial>>,
    primary_query: Query<&Window>,
) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DistortionMaterial {
    alpha_mode: AlphaMode,
    #[uniform(0)]
    vertex_uniforms: VertexUniforms,
    #[uniform(0)]
    fragment_uniforms: FragmentUniforms,
    #[texture(1)]
    #[sampler(2)]
    pub noise_view_x: Option<Handle<Image>>,
}

impl Material for DistortionMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders_ex/distortion.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders_ex/distortion.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

#[derive(Debug, Clone, ShaderType)]
pub struct FragmentUniforms {
    distortion_view: f32,
    speed_view: f32,
    fresnel_amount: f32,
    tint_color: Color,
}

impl Default for FragmentUniforms {
    fn default() -> Self {
        FragmentUniforms {
            distortion_view: 0.3,
            speed_view: 0.5,
            fresnel_amount: 0.0,
            tint_color: Color::rgb(0.0, 0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, ShaderType)]
struct VertexUniforms {
    distortion_vertex: f32,
    speed_vertex: f32,
}

impl Default for VertexUniforms {
    fn default() -> Self {
        VertexUniforms {
            distortion_vertex: 0.03,
            speed_vertex: 0.1,
        }
    }
}
