#import bevy_pbr::mesh_vertex_output  MeshVertexOutput
#import bevy_pbr::mesh_view_bindings  globals

@fragment
fn fragment(mesh: MeshVertexOutput) -> @location(0) vec4<f32> {
    let pos = mesh.world_position.xyz;
    let height = pos.y - 0.8;
    let speed = 2.0;
    let distance_from_top = sin(globals.time * speed) * 0.2 + 0.8;
    let min_red = 0.0;
    let max_red = 0.9;

    let red = mix(max_red, min_red, height + distance_from_top);
    let green = 1.0;
    let blue = 0.7;
    let alpha = 0.7;

    return vec4<f32>(red, green, blue, alpha);
}