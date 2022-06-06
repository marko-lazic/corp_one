use std::ops::Deref;

use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::asset::asset_loading::MaterialAsset;
use crate::world::colony::asset::color::ColorAsset;
use crate::world::colony::zone::ZoneType;
use crate::world::colony::Colony;

#[derive(Default, Deserialize, Debug, TypeUuid)]
#[uuid = "962DF4C2-C221-4364-A9F7-B7340FB60437"]
pub struct ColonyAsset {
    pub name: Colony,
    pub description: String,
    pub zones: Vec<ZoneAsset>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct EnergyNodeAsset {
    pub position: Vec3,
    pub material: MaterialAsset,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct VortexNodeAsset {
    pub position: Vec3,
}

impl Deref for VortexNodeAsset {
    type Target = VortexNodeAsset;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

#[derive(Default, Debug, Deserialize, Copy, Clone)]
pub struct VortexGateAsset {
    pub position: Vec3,
}

#[derive(Default, Debug, Deserialize, Copy, Clone)]
pub struct ZoneAsset {
    pub position: Vec3,
    pub value: f32,
    pub second: f32,
    pub zone_type: ZoneType,
    pub size: f32,
    pub material: MaterialAsset,
}

#[derive(Default, Debug, Deserialize, Copy, Clone)]
pub struct LightAsset {
    pub position: Vec3,
    pub color: ColorAsset,
}
