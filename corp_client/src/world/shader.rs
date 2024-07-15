use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

use crate::state::GameState;

#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
pub struct BarrierMaterial {
    alpha_mode: AlphaMode,
}

impl Material for BarrierMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/barrier.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct Shaders {
    pub(crate) barrier: Handle<BarrierMaterial>,
}

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BarrierMaterial>::default())
            .add_systems(OnExit(GameState::Loading), setup_shader);
    }
}

fn setup_shader(mut commands: Commands, mut barrier_materials: ResMut<Assets<BarrierMaterial>>) {
    commands.insert_resource(Shaders {
        barrier: barrier_materials.add(BarrierMaterial {
            alpha_mode: AlphaMode::Blend,
        }),
    });
}
