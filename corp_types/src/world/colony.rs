use serde::{Deserialize, Serialize};

#[derive(
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

    pub fn to_snake_case(&self) -> &'static str {
        match self {
            Colony::StarMap => "star_map",
            Colony::Cloning => "cloning",
            Colony::Iris => "iris",
            Colony::Liberte => "liberte",
            Colony::Playground => "playground",
        }
    }
}
