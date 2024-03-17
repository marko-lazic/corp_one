pub struct AssetPaths {
    pub default_font: &'static str,
    pub audio_slow_travel: &'static str,
    pub audio_walk: &'static str,
    pub mesh_energy_node: &'static str,
    pub mesh_cube: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    default_font: "fonts/Anonymous Pro.ttf",
    audio_slow_travel: "sound/slow-travel.wav",
    audio_walk: "sound/walk.wav",
    mesh_energy_node: "mesh/node/node_template.gltf#Mesh0/Primitive0",
    mesh_cube: "mesh/cube/cube.gltf#Mesh0/Primitive0",
};
