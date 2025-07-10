use bevy::prelude::Component;
use std::fmt::{Display, Formatter};

#[derive(Component, Debug)]
pub struct Username(pub String);

impl Display for Username {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
