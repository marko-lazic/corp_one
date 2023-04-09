use bevy::prelude::*;

#[derive(Component)]
pub struct Item {
    pub name: String,
}

impl Item {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}