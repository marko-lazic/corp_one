use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Dead;

#[derive(Event, Deserialize, Serialize, Clone, Debug)]
pub struct SendDeadPlayerToCloningCommand;

#[derive(Event, Deserialize, Serialize, Clone, Debug)]
pub struct KillMeCommand;
