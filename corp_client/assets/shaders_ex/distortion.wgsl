#import bevy_pbr::{
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip},
    mesh_view_bindings::globals,
}

struct VertexUniforms {
    distortion_vertex: f32,
    speed_vertex: f32,
}

struct FragmentUniforms {
    distortion_view: f32,
    speed_view: f32,
    fesnel_amount: f32,
    tint_color: vec3<f32>,
};

struct DistortionMaterial {
    vertex_uniforms: VertexUniforms,
    fragment_uniforms: FragmentUniforms,
}

@group(2) @binding(0)
var<uniform> distortion: DistortionMaterial;

@group(2) @binding(1)
var noise_view_x : texture_2d<f32>;
@group(2) @binding(2)
var noise_view_x_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let model = get_model_matrix(vertex.instance_index);
    out.position = mesh_position_local_to_clip(
        model,
        vec4<f32>(vertex.position, 1.0)
    );
    out.uv = vertex.uv;
    out.normal = vertex.normal;
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let speed_view = distortion.fragment_uniforms.speed_view;
    let uv = vec2<f32>(fract(input.uv.x + globals.time * speed_view), input.uv.y);

    let nts = textureSample(noise_view_x, noise_view_x_sampler, uv);

    let blue_color = vec4<f32>(0.0, 0.0, 0.2, 1.0);
    let final_color = nts + blue_color;

    return final_color;
}