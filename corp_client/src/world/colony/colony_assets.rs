use std::ops::Deref;

use bevy::{prelude::*, reflect::TypeUuid};
use rand::prelude::SliceRandom;
use serde::Deserialize;

use crate::asset::asset_loading::MaterialAsset;
use crate::world::colony::asset::color::ColorAsset;
use crate::world::colony::zone::ZoneType;

#[derive(Default, Deserialize, Debug, TypeUuid)]
#[uuid = "962DF4C2-C221-4364-A9F7-B7340FB60437"]
pub struct ColonyAsset {
    pub name: String,
    pub energy_nodes: Vec<EnergyNodeAsset>,
    pub vortex_gates: Vec<VortexGateAsset>,
    pub vortex_nodes: Vec<VortexNodeAsset>,
    pub zones: Vec<ZoneAsset>,
    pub lights: Vec<LightAsset>,
}

impl ColonyAsset {
    pub fn random_vortex_node_position(&self) -> Option<Vec3> {
        let vortex_node_positions = self
            .vortex_nodes
            .iter()
            .map(|n| n.position)
            .collect::<Vec<Vec3>>();
        vortex_node_positions
            .choose(&mut rand::thread_rng())
            .map(|p| p.to_owned())
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_vortex_node_position() {
        // given
        let expected_position = Vec3::new(0.0, 0.0, 0.0);
        let vortex_nodes = vec![VortexNodeAsset {
            position: expected_position,
        }];
        let colony_asset = ColonyAsset {
            vortex_nodes,
            ..Default::default()
        };

        // when
        let position = colony_asset.random_vortex_node_position();

        // then
        assert!(position.is_some(), "Position is cloned");
        assert_eq!(
            colony_asset.vortex_nodes[0].position, expected_position,
            "And position has not moved"
        )
    }
}
