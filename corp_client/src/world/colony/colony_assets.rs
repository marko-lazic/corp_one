use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::world::colony::asset_color::AssetColor;
use crate::world::zone::ZoneType;

#[derive(Deserialize, Debug, TypeUuid)]
#[uuid = "962DF4C2-C221-4364-A9F7-B7340FB60437"]
pub struct ColonyAsset {
    pub name: String,
    pub energy_nodes: Vec<EnergyNodeAsset>,
    pub vortex_gates: Vec<VortexGateAsset>,
    pub vortex_nodes: Vec<VortexNodeAsset>,
    pub zones: Vec<ZoneAsset>,
    pub lights: Vec<LightAsset>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct EnergyNodeAsset {
    pub position: Vec3,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct VortexNodeAsset {
    pub position: Vec3,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct VortexGateAsset {
    pub position: Vec3,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct ZoneAsset {
    pub position: Vec3,
    pub zone_type: ZoneType,
    pub size: f32,
    pub base_color: AssetColor,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct LightAsset {
    pub position: Vec3,
    pub color: AssetColor,
}