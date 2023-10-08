use std::time::Duration;

use bevy::{
    asset::ChangeWatcher,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Tell the asset server to watch for asset changes on disk:
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (640.0, 480.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
            MaterialPlugin::<DistortionMaterial>::default(),
            PanOrbitCameraPlugin,
            // BarrierPipelinePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
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
        mesh: meshes.add(shape::Cube::new(1.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        // material: custom_materials.add(CustomMaterial {
        //     fragment_uniforms: Default::default(),
        //     vertex_uniforms: Default::default(),
        //     alpha_mode: AlphaMode::Blend,
        //     noise_view_x: Some(asset_server.load("shaders_ex/noise/noise_texture_1.png")),
        //     noise_view_y: Some(asset_server.load("shaders_ex/noise/noise_texture_2.png")),
        //     noise_vertex: Some(asset_server.load("shaders_ex/noise/noise_texture_3.png")),
        // }
        material: custom_materials.add(DistortionMaterial {}),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: standard_materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn rotate_camera(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
    mouse_button_input: Res<Input<MouseButton>>,
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

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "878acc0e-9933-4b48-87c7-5326d3484d87"]
pub struct DistortionMaterial {
    #[uniform(0)]
    fragment_uniforms: FragmentUniforms,
    #[uniform(0)]
    vertex_uniforms: VertexUniforms,
    alpha_mode: AlphaMode,
    #[texture(1)]
    #[sampler(2)]
    pub noise_view_x: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub noise_view_y: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    pub noise_vertex: Option<Handle<Image>>,
}

impl Material for DistortionMaterial {
    // fn vertex_shader() -> ShaderRef {
    //     "shaders_ex/barrier_ex.wgsl".into()
    // }
    fn fragment_shader() -> ShaderRef {
        "shaders_ex/barrier_ex3.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}

#[derive(Debug, Clone, ShaderType)]
pub struct FragmentUniforms {
    distortion_view: f32,
    speed_view: f32,
    fesnel_amount: f32,
    tint_color: Color,
}

impl Default for FragmentUniforms {
    fn default() -> Self {
        FragmentUniforms {
            distortion_view: 0.3,
            speed_view: 0.5,
            fesnel_amount: 0.0,
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

#[derive(Resource)]
pub struct BarrierPipeline {
    texture_bind_group: BindGroupLayout,
}

impl FromWorld for BarrierPipeline {
    fn from_world(render_world: &mut World) -> Self {
        let texture_bind_group = render_world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("barrier_texture_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 7,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 8,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        BarrierPipeline { texture_bind_group }
    }
}

struct BarrierPipelinePlugin;

impl Plugin for BarrierPipelinePlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<BarrierPipeline>();
    }
}
