use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use corp_shared::prelude::*;

use crate::{
    state::GameState,
    world::colony::{barrier::BarrierPlugin, territory::TerritoryNodePlugin},
};

#[derive(Debug, Eq, PartialEq)]
pub enum Hover {
    Over,
    Out,
}

pub struct ColonyInteractionPlugin;

impl Plugin for ColonyInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackpackInteractionEvent>()
            .register_component_as::<dyn Interactive, Door>()
            .register_component_as::<dyn Interactive, TerritoryNode>()
            .add_plugins(BarrierPlugin)
            .add_plugins(TerritoryNodePlugin)
            .add_systems(
                Update,
                interaction_system.run_if(in_state(GameState::Playing)),
            );
    }
}
