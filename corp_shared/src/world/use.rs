use bevy::prelude::*;
#[derive(Component, Debug, Default)]
pub struct Use;

#[derive(Debug, Event)]
pub struct UseCommand {
    pub user: Entity,
}

impl UseCommand {
    pub fn new(user: Entity) -> Self {
        Self { user }
    }
}
