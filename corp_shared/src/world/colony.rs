use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(
    Component,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
    Default,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
)]
pub enum Colony {
    #[default]
    StarMap,
    Cloning,
    Iris,
    Liberte,
    Playground,
}

impl Colony {
    pub fn is_star_map(&self) -> bool {
        *self == Colony::StarMap
    }
}
