use crate::constants::tick;
use bevy::prelude::*;

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
    pub velocity: Vec3,
}

impl Movement {
    pub fn update_velocity(&mut self, direction: Vec3) -> Vec3 {
        self.velocity = direction * self.speed * tick::TIME_STEP;
        self.velocity
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            acceleration: 10.0,
            speed: 14.0,
            velocity: Vec3::ZERO,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_velocity() {
        // given
        let mut movement = Movement::default();
        let f = std::f32::consts::FRAC_1_SQRT_2;
        let direction = Vec3::from((f, 0.0, -f));

        // when
        movement.update_velocity(direction);

        // then
        assert_eq!(movement.velocity, Vec3::new(0.16499159, 0.0, -0.16499159));
    }
}
