pub struct AssetPath {
    pub default_font: &'static str,
    pub prepass_shader: &'static str,
    pub force_field_shader: &'static str,
}

pub const ASSET_PATH: AssetPath = AssetPath {
    default_font: "fonts/Anonymous Pro.ttf",
    prepass_shader: "shaders/show_prepass.wgsl",
    force_field_shader: "shaders/force_field.wgsl",
};
