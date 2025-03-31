use bevy::prelude::*;
#[derive(Component, Debug, Default)]
pub struct Use;

#[derive(Debug, Event)]
pub struct UseEvent {
    pub user: Entity,
}

impl UseEvent {
    pub fn new(user: Entity) -> Self {
        Self { user }
    }
}
