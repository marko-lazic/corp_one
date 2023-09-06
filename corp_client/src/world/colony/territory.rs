use bevy::prelude::*;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::*;

use crate::{
    state::GameState,
    world::{ccc::UseEntity, colony::colony_interaction::Hover},
};

#[derive(Event)]
pub struct TerritoryNodePickingEvent(Entity, Hover);

impl From<ListenerInput<Pointer<Over>>> for TerritoryNodePickingEvent {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        TerritoryNodePickingEvent(event.target, Hover::Over)
    }
}

impl From<ListenerInput<Pointer<Out>>> for TerritoryNodePickingEvent {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        TerritoryNodePickingEvent(event.target, Hover::Out)
    }
}

pub struct TerritoryNodePlugin;

impl Plugin for TerritoryNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TerritoryNodePickingEvent>().add_systems(
            Update,
            (receive_territory_node_pickings.run_if(on_event::<TerritoryNodePickingEvent>()),)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn receive_territory_node_pickings(
    mut pickings: EventReader<TerritoryNodePickingEvent>,
    mut r_use_target: ResMut<UseEntity>,
) {
    for event in pickings.iter() {
        if event.1 == Hover::Over {
            r_use_target.set(Some(event.0));
        } else if event.1 == Hover::Out {
            r_use_target.set(None);
        }
    }
}
