use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Item;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[require(Item, Name::new("Hacking Tool"))]
#[cfg_attr(feature = "client", require(
    StateScoped<crate::prelude::GameState> = StateScoped(crate::prelude::GameState::Playing))
)]
pub struct HackingTool;
