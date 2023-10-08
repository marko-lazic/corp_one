#import bevy_pbr::mesh_vertex_output  MeshVertexOutput
#import bevy_pbr::mesh_view_bindings  globals

//// fragment uniforms
struct FragmentUniforms {
    distortion_view: f32,
    speed_view: f32,
    fesnel_amount: f32,
    tint_color: vec3<f32>,
};

struct VertexUniforms {
    distortion_vertex: f32,
    speed_vertex: f32,
}

struct CustomMaterial {
     fragment_uniforms: FragmentUniforms,
     vertex_uniforms: VertexUniforms,
}

@group(1) @binding(0)
var<uniform> custom_material : CustomMaterial;

// textures
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

@group(1) @binding(7)
var screen_texture : texture_2d<f32>;
@group(1) @binding(8)
var screen_texture_sampler: sampler;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// vertex output
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.position = input.position;
    let vertexUniforms = custom_material.vertex_uniforms;
    // Vertex shader logic here
    let uv_speed = input.uv + (globals.time * vertexUniforms.speed_vertex);
//    let noise_val = (textureSample(noise_vertex, noise_vertex_sampler, input.uv).r * 2.0) - 1.0;
    let noise_val = 0.1;
    // let noiseVal = (textureLoad(noise_vertex, input.uv + (globals.time * vertexUniforms.speed_vertex)).r * 2.0) - 1.0;
    let displacement = input.normal * noise_val * vertexUniforms.distortion_vertex;
    output.position += vec4<f32>(displacement, 0.0);

    output.uv = input.uv;
    output.normal = input.normal;
    return output;
}

@fragment
fn fragment_main(input: MeshVertexOutput) -> @location(0) vec4<f32> {
    // Fragment shader logic here
    let fragmentUniforms = custom_material.fragment_uniforms;
    let noiseValueX = (textureSample(noise_view_x, noise_view_x_sampler, input.uv + (globals.time * fragmentUniforms.speed_view)).r * 2.0) - 1.0;
    let noiseValueY = (textureSample(noise_view_y, noise_view_y_sampler, input.uv + (globals.time * fragmentUniforms.speed_view)).r * 2.0) - 1.0;
    let noiseDistort = vec2<f32>(noiseValueX, noiseValueY) * fragmentUniforms.distortion_view;

//    let distortedscreen_texture = textureSample(screen_texture, screen_texture_sampler, input.uv + noiseDistort).rgb;
    let distortedscreen_texture = vec3<f32>(0.0);

    let normal = normalize(input.normal);
    let view = normalize(-input.position.xyz);
    let fresnelTint = (fragmentUniforms.tint_color * pow((1.0 - clamp(dot(normal, view), 0.0, 1.0)), fragmentUniforms.fesnel_amount));

    return vec4<f32>(distortedscreen_texture + fresnelTint, 1.0);
      return vec4<f32>(0.5, 0.5, 0.5, 1.0);
}