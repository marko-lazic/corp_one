use bevy::prelude::Entity;

#[derive(Default)]
pub enum UseEntity {
    Barrier(Entity),
    #[default]
    None,
}


