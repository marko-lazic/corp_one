use bevy::asset::Handle;
use bevy::prelude::StandardMaterial;
use serde::Deserialize;

use crate::asset::asset_loading::MaterialAssets;

#[derive(Debug, Deserialize, Clone)]
pub enum MaterialAsset {
    Green,
    Blue,
    OrangeRed,
    SeaGreen,
    Unknown,
}

impl Default for MaterialAsset {
    fn default() -> Self {
        Self::Unknown
    }
}

impl MaterialAssets {
    pub fn get_material(&self, asset_material: &MaterialAsset) -> Handle<StandardMaterial> {
        match asset_material {
            MaterialAsset::Green => self.green_material.clone(),
            MaterialAsset::Blue => self.blue_material.clone(),
            MaterialAsset::OrangeRed => self.orange_red.clone(),
            MaterialAsset::SeaGreen => self.sea_green.clone(),
            MaterialAsset::Unknown => self.pink_material.clone(),
        }
    }
}
