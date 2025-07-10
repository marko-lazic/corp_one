use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Player;

#[derive(Event, Serialize, Deserialize)]
pub struct PlayerSpawnClientCommand;

/// A trigger from server that instructs the client to mark a specific entity as [`LocalPlayer`].
#[derive(Event, Serialize, Deserialize)]
pub struct SetupPlayerServerCommand;
