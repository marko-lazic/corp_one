struct BarrierMaterial {
 color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: BarrierMaterial;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return material.color;
}
