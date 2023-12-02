use bevy::prelude::*;

use corp_shared::prelude::{InteractionEvent, UseTerritoryNodeEvent};

use crate::{
    state::GameState,
    world::{
        ccc::UseEntity,
        colony::object_interaction::{Hover, PickingEvent},
    },
};

pub struct TerritoryNodePickingEvent;

pub struct TerritoryNodePlugin;

impl Plugin for TerritoryNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickingEvent<TerritoryNodePickingEvent>>()
            .add_event::<InteractionEvent<UseTerritoryNodeEvent>>()
            .add_systems(
                Update,
                (
                    receive_territory_node_pickings
                        .run_if(on_event::<PickingEvent<TerritoryNodePickingEvent>>()),
                    territory_interaction_system
                        .run_if(on_event::<InteractionEvent<UseTerritoryNodeEvent>>()),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn receive_territory_node_pickings(
    mut pickings: EventReader<PickingEvent<TerritoryNodePickingEvent>>,
    mut r_use_target: ResMut<UseEntity>,
) {
    for event in pickings.read() {
        if event.mode == Hover::Over {
            r_use_target.set(Some(event.target));
        } else if event.mode == Hover::Out {
            r_use_target.set(None);
        }
    }
}

fn territory_interaction_system(
    mut ev_interaction: EventReader<InteractionEvent<UseTerritoryNodeEvent>>,
) {
    for event in ev_interaction.read() {
        info!("Interaction with territory node: {:?}", event.target);
    }
}
