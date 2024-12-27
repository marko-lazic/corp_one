use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[require(
    Name(||  Name::new("Hacking Tool"))
)]
pub struct HackingTool;
