pub struct AssetPaths {
    pub font_fira_sans: &'static str,
    pub audio_slow_travel: &'static str,
    pub mesh_energy_node: &'static str,
    pub mesh_cube: &'static str,
    pub mesh_mannequiny: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    font_fira_sans: "fonts/FiraMono-Medium.ttf",
    audio_slow_travel: "audio/slow-travel.wav",
    mesh_energy_node: "mesh/node/node_template.gltf#Mesh0/Primitive0",
    mesh_cube: "mesh/cube/cube.gltf#Mesh0/Primitive0",
    mesh_mannequiny: "mesh/mannequiny/mannequiny-0.3.0.glb#Mesh0/Primitive0",
};
