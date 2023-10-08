#import bevy_pbr::mesh_view_bindings    view,globals
#import bevy_pbr::mesh_bindings         mesh
#import bevy_pbr::mesh_functions        mesh_position_local_to_clip

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

@group(1) @binding(0)
var<uniform> distortion: DistortionMaterial;

@group(1) @binding(1)
var noise_view_x : texture_2d<f32>;
@group(1) @binding(2)
var noise_view_x_sampler: sampler;

@group(1) @binding(3)
var noise_view_y : texture_2d<f32>;
@group(1) @binding(4)
var noise_view_y_sampler: sampler;

@group(1) @binding(5)
var noise_vertex : texture_2d<f32>;
@group(1) @binding(6)
var noise_vertex_sampler: sampler;

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var screen_texture_sampler: sampler;

struct VertexInput {
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
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(
        mesh.model,
        vec4<f32>(vertex.position, 1.0)
    );
    let speed_vertex = distortion.vertex_uniforms.speed_vertex;
    let uv = vec2<f32>(fract(vertex.uv.x + globals.time * speed_vertex), fract(vertex.uv.y + globals.time * speed_vertex));
    let noise_val = (textureLoad(noise_vertex, vec2<i32>(uv), 1).r * 2.0) - 1.0;
    let distortion_vertex = distortion.vertex_uniforms.distortion_vertex;
    let displacement: vec3<f32> = vertex.normal * noise_val * distortion_vertex;

    out.position = out.position + vec4<f32>(displacement, 0.0);
    out.uv = vertex.uv;
    out.normal = vertex.normal;
    return out;
}

fn speed_time(coord: f32) -> f32 {
    let speed_view = distortion.fragment_uniforms.speed_view;
    return fract(coord + globals.time * speed_view);
}

fn fresnel(amount: f32, normal: vec3<f32>, view: vec3<f32>) -> f32 {
    return pow(1.0 - clamp(dot(normalize(normal), normalize(view)), 0.0, 1.0), amount);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(speed_time(input.uv.x), speed_time(input.uv.y));
    let screen_uv = input.position.xy / view.viewport.zw;

    let noise_value_x = textureSample(noise_view_x, noise_view_x_sampler, uv).r * 2.0 - 1.0;
    let noise_value_y = textureSample(noise_view_y, noise_view_y_sampler, uv).r * 2.0 - 1.0;
    let noise_distort = vec2<f32>(noise_value_x, noise_value_y) * distortion.fragment_uniforms.distortion_view;
    let distorted_screen_texture = vec3<f32>(textureSample(screen_texture, screen_texture_sampler, screen_uv + noise_distort).rgb);
    let tint_color = distortion.fragment_uniforms.tint_color;
    let fesnel_amount = distortion.fragment_uniforms.fesnel_amount;
    let fesnel_tint = tint_color * fresnel(fesnel_amount, normalize(input.normal), view.world_position);

    return vec4<f32>(distorted_screen_texture * fesnel_tint, 1.0);
}