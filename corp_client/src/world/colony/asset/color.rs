use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum ColorAsset {
    White,
    OrangeRed,
    SeaGreen,
    LimeGreen,
    Red,
    None,
}

impl Default for ColorAsset {
    fn default() -> Self {
        Self::None
    }
}
