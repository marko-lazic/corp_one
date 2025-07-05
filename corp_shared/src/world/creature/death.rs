use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Dead;

#[derive(Event)]
pub struct YouDied;

#[derive(Event, Deserialize, Serialize, Clone, Debug)]
pub struct KillMeCommand;
