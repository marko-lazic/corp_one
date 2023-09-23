#import bevy_pbr::mesh_vertex_output    MeshVertexOutput
#import bevy_pbr::mesh_view_bindings    view
#import bevy_pbr::mesh_view_bindings    globals

struct StarfieldMaterial {
    mouse: vec2<f32>,
    speed2: f32,
}

@group(1) @binding(0)
var<uniform> starfield: StarfieldMaterial;

const iterations: i32 = 12;
const formuparam2: f32 = 0.79;
const volsteps: f32 = 7.0;
const stepsize: f32 = 0.290;
const zoom: f32 = 1.0;
const tile: f32 = 0.850;

const brightness: f32 = 0.0015;
const darkmatter: f32 = 0.100;
const distfading: f32 = 0.560;
const saturation: f32 = 0.90;

const transversespeed: f32 = 1.0; // zoom
const cloud: f32 = 0.17;


fn triangle2(x: f32, a: f32) -> f32 {
    let output2 = 2.0 * abs(3.0 * ((x/a) - floor(x/a + 0.5))) - 1.0;
    return output2;
}

struct FieldResult {
    p: vec3<f32>,
    field: f32,
}

fn field(p: vec3<f32>) -> FieldResult {
	let strength = 7. + .03 * log(1.e-6 + fract(sin(globals.time) * 373.11));
	var accum = 0.;
	var prev = 0.;
	var tw = 0.;
	var new_p: vec3<f32> = vec3<f32>(0.0);

	for (var i: i32 = 0; i < 6; i++) {
		let mag = dot(p, p);
		new_p = abs(p) / mag + vec3(-.5, -.8 + 0.1*sin(-globals.time*0.1 + 2.0), -1.1+0.3*cos(globals.time*0.3));
		let w = exp(-f32(i+1) / 7.);
		accum += w * exp(-strength * pow(abs(mag - prev), 2.3));
		tw += w;
		prev = mag;
	}

	var field_result: FieldResult;
    field_result.p = new_p;
    field_result.field = max(0., 5. * accum / tw - .7);

	return field_result;
}

@fragment
fn fragment(mesh: MeshVertexOutput) -> @location(0) vec4<f32> {
    let mouse_x = starfield.mouse.x / view.viewport.z;
    let mouse_y = starfield.mouse.y / view.viewport.w;

    return vec4<f32>(mouse_x, mouse_y, 0.0, 1.0);
}