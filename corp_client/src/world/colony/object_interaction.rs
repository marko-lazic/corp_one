use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use corp_shared::prelude::*;

use crate::{
    state::GameState,
    world::colony::{barrier::BarrierPlugin, territory::TerritoryNodePlugin},
};

#[derive(Event)]
pub struct PickingEvent<E> {
    pub target: Entity,
    pub mode: Hover,
    _marker: PhantomData<E>,
}

impl<E> From<ListenerInput<Pointer<Over>>> for PickingEvent<E> {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        PickingEvent {
            target: event.target,
            mode: Hover::Over,
            _marker: PhantomData,
        }
    }
}

impl<E> From<ListenerInput<Pointer<Out>>> for PickingEvent<E> {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        PickingEvent {
            target: event.target,
            mode: Hover::Out,
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Hover {
    Over,
    Out,
}

pub struct ObjectInteractionPlugin;

impl Plugin for ObjectInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackpackInteractionEvent>() // Does not have plugin ATM
            .add_plugins(BarrierPlugin)
            .add_plugins(TerritoryNodePlugin)
            .add_systems(
                Update,
                interaction_system.run_if(in_state(GameState::Playing)),
            );
    }
}
