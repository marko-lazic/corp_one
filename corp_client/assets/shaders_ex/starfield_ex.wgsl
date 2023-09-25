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

const transverse_speed: f32 = 1.0; // zoom
const cloud: f32 = 0.17;


fn triangle2(x: f32, a: f32) -> f32 {
    let output2 = 2.0 * abs(3.0 * ((x / a) - floor(x / a + 0.5))) - 1.0;
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
        new_p = abs(p) / mag + vec3(-.5, -.8 + 0.1 * sin(-globals.time * 0.1 + 2.0), -1.1 + 0.3 * cos(globals.time * 0.3));
        let w = exp(-f32(i + 1) / 7.);
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
    let uv2 = 2. * mesh.position.xy / vec2<f32>(512.) - 1.;
    let uvs = uv2 * vec2<f32>(512.) / 512.;

    let time2 = globals.time;
    var speed = -starfield.speed2;
    speed = .005 * cos(time2 * 0.02 + 3.1415926 / 4.0);
    let formuparam = formuparam2;

    //get coords and direction
    let uv = uvs;
    //mouse rotation
    let a_xz = 0.9;
    let a_yz = -.6;
    let a_xy = 0.9 + globals.time * 0.08;

    let rot_xz = mat2x2<f32>(vec2(cos(a_xz), sin(a_xz)), vec2(-sin(a_xz), cos(a_xz)));
    let rot_yz = mat2x2<f32>(vec2(cos(a_yz), sin(a_yz)), vec2(-sin(a_yz), cos(a_yz)));
    let rot_xy = mat2x2<f32>(vec2(cos(a_xy), sin(a_xy)), vec2(-sin(a_xy), cos(a_xy)));

    let v2 = 1.0;
    var dir = vec3<f32>(uv * zoom, 1.);
    var v_from = vec3<f32>(0.0, 0.0, 0.0);
    v_from.x -= 2.0 * (starfield.mouse.x - 0.5);
    v_from.y -= 2.0 * (starfield.mouse.y - 0.5);

    var forward = vec3(0., 0., 1.);
    v_from.x += transverse_speed * (1.0) * cos(0.01 * globals.time) + 0.001 * globals.time;
    v_from.y += transverse_speed * (1.0) * sin(0.01 * globals.time) + 0.001 * globals.time;
    v_from.z += 0.003 * globals.time;

//    dir.xy *= rot_xy;
//    forward.xy *= rot_xy;
//    dir.xz *= rot_xz;
//    forward.xz *= rot_xz;
//    dir.yz *= rot_yz;
//    forward.yz *= rot_yz;

    let dir_xy = dir.xy * rot_xy;
    dir = vec3(dir_xy[0], dir_xy[1], dir.z);
    let forward_xy = forward.xy * rot_xy;
    forward = vec3(forward_xy[0], forward_xy[1], forward.z);
    let dir_xz = dir.xz * rot_xz;
    dir = vec3(dir_xz[0], dir.y, dir_xz[1]);
    let forward_xz = forward.xz * rot_xz;
    forward = vec3(forward_xz[0], forward.y, forward_xz[1]);
    let dir_yz = dir.yz * rot_yz;
    dir = vec3(dir.x, dir_yz[0], dir_yz[1]);
    let forward_yz = forward.yz * rot_yz;
    forward = vec3(forward.x, forward_yz[0], forward_yz[1]);

//    v_from.xy *= -1.0 * rot_xy;
//    v_from.xz *= rot_xz;
//    v_from.yz *= rot_yz;

    let new_from_xy: vec2<f32> = v_from.xy * -1.0 * rot_xy;
    v_from = vec3(new_from_xy[0], new_from_xy[1], v_from.z);
    let new_from_xz: vec2<f32> = v_from.xz * rot_xz;
    v_from = vec3(new_from_xz[0], v_from.y, new_from_xz[1]);
    let new_from_yz = v_from.yz * rot_yz;
    v_from = vec3(v_from.x, new_from_yz[0], new_from_yz[1]);

    //zoom
    let zooom = (time2 - 3311.) * speed;
    v_from += forward * zooom;
    var sampleShift = zooom % stepsize;

    let zoffset = -sampleShift;
    sampleShift /= stepsize; // make from 0 to 1

    //volumetric rendering
    var s = 0.24;
    var s3 = s + stepsize / 2.0;
    var v = vec3(0.);
    var t3 = 0.0;

    var backCol2 = vec3(0.);
    for (var r: i32 = 0; r < i32(volsteps); r++) {
        var p2 = v_from + (s + zoffset) * dir; // + vec3(0.,0.,zoffset);
        var p3 = v_from + (s3 + zoffset) * dir; // + vec3(0.,0.,zoffset);

        p2 = abs(vec3(tile) - p2 % vec3(tile * 2.)); // tiling fold
        p3 = abs(vec3(tile) - p3 % vec3(tile * 2.)); // tiling fold
        // #ifdef cloud
        let field_result: FieldResult = field(p3);
        p3 = field_result.p;
        t3 = field_result.field;

        var pa: f32 = 0.0;
        var a: f32 = pa;

        for (var i: i32 = 0; i < iterations; i++) {
            p2 = abs(p2) / dot(p2, p2) - formuparam; // the magic formula
            //p=abs(p)/max(dot(p,p),0.005)-formuparam; // another interesting way to reduce noise
            let D = abs(length(p2) - pa); // absolute sum of average change
            var a: f32 = 0.0;
            if (i > 7) {
                a += min(12.0, D);
            } else {
                a += D;
            }
            pa = length(p2);
        }


        //float dm=max(0.,darkmatter-a*a*.001); //dark matter
        a = a * (a * a); // add contrast
        //if (r>3) fade*=1.-dm; // dark matter, don't render near
        // brightens stuff up a bit
        var s1 = s + zoffset;
        // need closed form expression for this, now that we shift samples
        var fade = pow(distfading, max(0., f32(r) - sampleShift));
        //t3 += fade;
        v += fade;
        //backCol2 -= fade;

        // fade out samples as they approach the camera
        if r == 0 {
            fade *= (1.0 - sampleShift);
        }
        // fade in samples as they approach from the distance
        if r == i32(volsteps) - 1 {
            fade *= sampleShift;
        }
        v += vec3(s1, s1 * s1, s1 * s1 * s1 * s1) * a * brightness * fade; // coloring based on distance

        backCol2 += mix(.4, 1., v2) * vec3(1.8 * t3 * t3 * t3, 1.4 * t3 * t3, t3) * fade;

        s += stepsize;
        s3 += stepsize;
    }

    v = mix(vec3(length(v)), v, saturation); //color adjust

    let forCol2 = vec4(v * .01, 1.);
    backCol2 *= cloud;
    backCol2.b *= 1.8;
    backCol2.r *= 0.05;

    backCol2.b = 0.5 * mix(backCol2.g, backCol2.b, 0.8);
    backCol2.g = 0.0;
    backCol2.b = mix(backCol2.b, backCol2.g, 0.5 * (cos(globals.time * 0.01) + 1.0));
    backCol2.g = mix(backCol2.g, backCol2.b, 0.5 * (cos(globals.time * 0.01) + 1.0));

    return forCol2 + vec4(backCol2, 1.0);
}