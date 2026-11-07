use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct AuthToken(pub String);

impl Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
