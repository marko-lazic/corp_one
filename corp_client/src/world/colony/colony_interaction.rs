use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use corp_shared::prelude::*;

use crate::state::GameState;
use crate::world::colony::barrier::BarrierPlugin;

pub struct ColonyInteractionPlugin;

impl Plugin for ColonyInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackpackInteractionEvent>();
        app.register_component_as::<dyn Interactive, Door>();
        app.add_plugin(BarrierPlugin);
        app.add_system((interaction_system).in_set(OnUpdate(GameState::Playing)));
    }
}
