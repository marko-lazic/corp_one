use bevy::render::color::Color as BevyColor;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum AssetColor {
    OrangeRed,
    SeaGreen,
    LimeGreen,
    Red,
    Unknwon,
}

impl Default for AssetColor {
    fn default() -> Self {
        Self::Unknwon
    }
}

impl AssetColor {
    pub fn get_color(&self) -> BevyColor {
        match self {
            AssetColor::OrangeRed => BevyColor::ORANGE_RED,
            AssetColor::SeaGreen => BevyColor::SEA_GREEN,
            AssetColor::LimeGreen => BevyColor::LIME_GREEN,
            AssetColor::Red => BevyColor::RED,
            AssetColor::Unknwon => BevyColor::PINK,
        }
    }
}