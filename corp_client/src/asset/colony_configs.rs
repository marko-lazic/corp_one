use bevy::{prelude::*, reflect::TypePath};
use serde::Deserialize;

#[derive(Asset, Clone, Default, Deserialize, Debug, TypePath)]
pub struct ColonyConfig {
    pub name: Colony,
    #[allow(dead_code)]
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
