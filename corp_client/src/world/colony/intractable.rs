use bevy::prelude::Entity;

pub enum UseEntity {
    Barrier(Entity),
    None,
}

impl Default for UseEntity {
    fn default() -> Self {
        UseEntity::None
    }
}
