use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use corp_shared::prelude::*;

use crate::{state::GameState, world::colony::barrier::BarrierPlugin};

pub struct ColonyInteractionPlugin;

impl Plugin for ColonyInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackpackInteractionEvent>();
        app.register_component_as::<dyn Interactive, Door>();
        app.add_plugins(BarrierPlugin);
        app.add_systems(
            Update,
            interaction_system.run_if(in_state(GameState::Playing)),
        );
    }
}
