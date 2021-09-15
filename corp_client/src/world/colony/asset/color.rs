use bevy::render::color::Color as BevyColor;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum ColorAsset {
    White,
    OrangeRed,
    SeaGreen,
    LimeGreen,
    Red,
    Unknwon,
}

impl Default for ColorAsset {
    fn default() -> Self {
        Self::Unknwon
    }
}

impl ColorAsset {
    pub fn get_color(&self) -> BevyColor {
        match self {
            ColorAsset::White => BevyColor::WHITE,
            ColorAsset::OrangeRed => BevyColor::ORANGE_RED,
            ColorAsset::SeaGreen => BevyColor::SEA_GREEN,
            ColorAsset::LimeGreen => BevyColor::LIME_GREEN,
            ColorAsset::Red => BevyColor::RED,
            ColorAsset::Unknwon => BevyColor::PINK,
        }
    }
}
