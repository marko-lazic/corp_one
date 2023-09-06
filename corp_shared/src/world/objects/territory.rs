use bevy::prelude::*;

use crate::prelude::{InteractionType, Interactive, Security};

pub enum TerritoryNodeType {
    EnergyNode,
    PowerPlant,
    ControlCenter,
}

#[derive(Component)]
pub struct TerritoryNode {
    pub r#type: TerritoryNodeType,
    pub security: Security,
}

impl Interactive for TerritoryNode {
    fn interaction_type(&self) -> InteractionType {
        InteractionType::TerritoryNode
    }
}
