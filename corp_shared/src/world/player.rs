use bevy::prelude::*;
use bevy_replicon::prelude::Replicated;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[require(Replicated)]
pub struct Player;

#[derive(Event, Serialize, Deserialize)]
pub struct ClientPlayerSpawnCommand;

/// A trigger that instructs the client to mark a specific entity as [`LocalPlayer`].
#[derive(Event, Serialize, Deserialize)]
pub struct MakeLocal;
