use bevy::{ecs::entity::MapEntities, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Player;

#[derive(Event, Serialize, Deserialize, Clone, Copy)]
pub struct PlayerSpawnCommand;

/// A trigger from server that instructs the client to mark a specific entity as [`LocalPlayer`].
#[derive(Event, Serialize, Deserialize, MapEntities)]
pub struct SetupPlayerCommand {
    #[entities]
    pub player_target: Entity,
}
