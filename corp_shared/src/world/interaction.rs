use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct UseEvent {
    pub user: Entity,
}

impl UseEvent {
    pub fn new(user: Entity) -> Self {
        Self { user }
    }
}

#[derive(Component, Debug)]
pub enum InteractionObjectType {
    DoorControl,
    TerritoryNode,
    Backpack,
}
