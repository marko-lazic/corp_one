use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player;

#[derive(Event, Serialize, Deserialize)]
pub struct ClientPlayerSpawnCommand;

/// A trigger that instructs the client to mark a specific entity as [`LocalPlayer`].
#[derive(Event, Serialize, Deserialize)]
pub struct MakeLocal;
