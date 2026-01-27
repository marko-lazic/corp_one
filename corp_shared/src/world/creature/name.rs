use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct CreatureName(pub String);

impl Display for CreatureName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
