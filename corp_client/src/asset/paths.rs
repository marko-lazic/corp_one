pub struct AssetPaths {
    pub font_fira_sans: &'static str,
    pub audio_slow_travel: &'static str,
    pub audio_walk: &'static str,
    pub mesh_energy_node: &'static str,
    pub mesh_cube: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    font_fira_sans: "fonts/FiraMono-Medium.ttf",
    audio_slow_travel: "sound/slow-travel.wav",
    audio_walk: "sound/walk.wav",
    mesh_energy_node: "mesh/node/node_template.gltf#Mesh0/Primitive0",
    mesh_cube: "mesh/cube/cube.gltf#Mesh0/Primitive0",
};
