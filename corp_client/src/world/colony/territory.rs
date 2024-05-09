use bevy::prelude::*;

use corp_shared::prelude::{InteractionEvent, UseTerritoryNodeEvent};

use crate::state::GameState;

pub struct TerritoryNodePlugin;

impl Plugin for TerritoryNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionEvent<UseTerritoryNodeEvent>>()
            .add_systems(
                Update,
                (territory_interaction_system
                    .run_if(on_event::<InteractionEvent<UseTerritoryNodeEvent>>()),)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn territory_interaction_system(
    mut ev_interaction: EventReader<InteractionEvent<UseTerritoryNodeEvent>>,
) {
    for event in ev_interaction.read() {
        info!("Interaction with territory node: {:?}", event.target);
    }
}
