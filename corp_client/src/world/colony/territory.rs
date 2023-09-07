use bevy::prelude::*;
use bevy_trait_query::RegisterExt;
use corp_shared::prelude::{Interactive, TerritoryNode};

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
            .register_component_as::<dyn Interactive, TerritoryNode>()
            .add_systems(
                Update,
                (receive_territory_node_pickings
                    .run_if(on_event::<PickingEvent<TerritoryNodePickingEvent>>()))
                .run_if(in_state(GameState::Playing)),
            );
    }
}

fn receive_territory_node_pickings(
    mut pickings: EventReader<PickingEvent<TerritoryNodePickingEvent>>,
    mut r_use_target: ResMut<UseEntity>,
) {
    for event in pickings.iter() {
        if event.mode == Hover::Over {
            r_use_target.set(Some(event.target));
        } else if event.mode == Hover::Out {
            r_use_target.set(None);
        }
    }
}
