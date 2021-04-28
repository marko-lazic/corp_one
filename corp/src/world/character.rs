use bevy::prelude::Bundle;

pub const EMPTY_CHARACTER_NAME: &str = "";

pub struct CharacterName(String);

impl CharacterName {
    pub fn new(name: &str) -> Self {
        CharacterName(String::from(name))
    }
}

pub struct Health(u8);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}

#[derive(Debug)]
pub struct Movement {
    pub acceleration: f32,
    pub speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            acceleration: 12.0,
            speed: 400.0,
        }
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: CharacterName,
    pub health: Health,
    pub movement: Movement,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            name: CharacterName(EMPTY_CHARACTER_NAME.to_string()),
            health: Default::default(),
            movement: Default::default(),
        }
    }
}
