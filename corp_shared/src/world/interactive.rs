use bevy::prelude::*;

pub trait InteractionEntity {}

#[derive(Component)]
pub enum InteractionObjectType {
    Door,
    TerritoryNode,
}

#[derive(Event, PartialEq, Copy, Clone, Debug)]
pub struct InteractionEvent<T> {
    pub interactor: Entity,
    pub target: Entity,
    _interaction_type: std::marker::PhantomData<T>,
}

impl<T> InteractionEvent<T> {
    pub fn new(interactor: Entity, target: Entity, _interaction_type: T) -> Self {
        InteractionEvent {
            interactor,
            target,
            _interaction_type: std::marker::PhantomData,
        }
    }
}
