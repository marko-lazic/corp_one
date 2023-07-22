use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

use crate::asset::MaterialAsset;

#[derive(Default, Deserialize, Debug, TypeUuid, TypePath)]
#[uuid = "962DF4C2-C221-4364-A9F7-B7340FB60437"]
pub struct ColonyConfig {
    pub name: Colony,
    pub description: String,
    pub zones: Vec<ZoneConfig>,
}

#[derive(Default, Debug, Deserialize, Copy, Clone)]
pub struct ZoneConfig {
    pub position: Vec3,
    pub value: f32,
    pub second: f32,
    pub zone_type: ZoneType,
    pub size: f32,
    pub material: MaterialAsset,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ZoneType {
    Heal,
    Damage,
    Unknown,
}

impl Default for ZoneType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum Colony {
    Cloning,
    Iris,
    Liberte,
    Playground,
}

impl Default for Colony {
    fn default() -> Self {
        Self::Cloning
    }
}
