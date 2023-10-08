use std::time::Duration;

use bevy::{
    asset::ChangeWatcher,
    core_pipeline::{core_3d, fullscreen_vertex_shader::fullscreen_shader_vertex_state},
    ecs::query::QueryItem,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::MeshVertexBufferLayout,
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            AsBindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState,
            Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment,
            RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType,
            SamplerDescriptor, ShaderRef, ShaderStages, ShaderType, SpecializedMeshPipelineError,
            TextureFormat, TextureSampleType, TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
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
            PanOrbitCameraPlugin,
            MaterialPlugin::<DistortionMaterial>::default(),
            DistortionShaderPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, update_distortion))
        .run();
}

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
    ));
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Cube::new(1.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: custom_materials.add(DistortionMaterial {
            alpha_mode: AlphaMode::Blend,
            vertex_uniforms: Default::default(),
            fragment_uniforms: Default::default(),
            noise_view_x: Some(asset_server.load("shaders_ex/noise/noise_texture_1.png")),
            noise_view_y: Some(asset_server.load("shaders_ex/noise/noise_texture_2.png")),
            noise_vertex: Some(asset_server.load("shaders_ex/noise/noise_texture_3.png")),
        }),
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
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
    time: Res<Time>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        let pan_orbit = pan_orbit_query.single_mut().into_inner();
        pan_orbit.target_alpha += 15f32.to_radians() * time.delta_seconds();
    }
}

fn update_distortion(
    material_handle: Query<&Handle<DistortionMaterial>>,
    mut materials: ResMut<Assets<DistortionMaterial>>,
    primary_query: Query<&Window>,
) {
    let Ok(_primary) = primary_query.get_single() else {
        return;
    };
    let handle = material_handle.single();
    let _mat = materials.get_mut(handle).unwrap();
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
pub struct DistortionMaterial {
    alpha_mode: AlphaMode,
    #[uniform(0)]
    vertex_uniforms: VertexUniforms,
    #[uniform(0)]
    fragment_uniforms: FragmentUniforms,
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

struct DistortionShaderPlugin;

impl Plugin for DistortionShaderPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((
        //     ExtractComponentPlugin::<GBShaderSettings>::default(),
        //     UniformComponentPlugin::<GBShaderSettings>::default(),
        // ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(
                core_3d::graph::NAME,
                PostProcessNode::NAME,
            )
            .add_render_graph_edges(
                core_3d::graph::NAME,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    PostProcessNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<PostProcessPipeline>();
    }
}

#[derive(Default)]
struct PostProcessNode;

impl PostProcessNode {
    pub const NAME: &'static str = "post_process";
}

impl ViewNode for PostProcessNode {
    type ViewQuery = &'static ViewTarget;

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_target: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        // let settings_uniforms = world.resource::<ComponentUniforms<GBShaderSettings>>();
        // let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
        //     return Ok(());
        // };

        let post_process = view_target.post_process_write();

        let bind_group = render_context
            .render_device()
            .create_bind_group(&BindGroupDescriptor {
                label: Some("post_process_bind_group"),
                layout: &post_process_pipeline.layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(post_process.source),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&post_process_pipeline.sampler),
                    },
                    // BindGroupEntry {
                    //     binding: 2,
                    //     resource: settings_binding.clone(),
                    // },
                ],
            });

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("post_process_bind_group_layout"),
            entries: &[
                // The screen texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // The sampler that will be used to sample the screen texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // The settings uniform that will control the effect
                // BindGroupLayoutEntry {
                //     binding: 2,
                //     visibility: ShaderStages::FRAGMENT,
                //     ty: BindingType::Buffer {
                //         ty: bevy::render::render_resource::BufferBindingType::Uniform,
                //         has_dynamic_offset: false,
                //         min_binding_size: None,
                //     },
                //     count: None,
                // },
            ],
        });

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world
            .resource::<AssetServer>()
            .load("shaders_ex/distortion.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("post_process_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
