use bevy::prelude::*;
#[derive(Component, Debug, Default)]
pub struct Use;

#[derive(Debug, EntityEvent)]
pub struct UseCommand {
    #[event_target]
    pub usable: Entity,
    pub user: Entity,
}

impl UseCommand {
    pub fn new(usable: Entity, user: Entity) -> Self {
        Self { usable, user }
    }
}
