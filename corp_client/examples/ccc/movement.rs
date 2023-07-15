use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ControlMovement {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct CharacterMovement {
    pub can_move: bool,
    pub direction: Vec3,
    pub velocity: Vec3,
    pub speed: f32,
}

impl Default for CharacterMovement {
    fn default() -> Self {
        Self {
            can_move: true,
            direction: Vec3::ZERO,
            velocity: Vec3::ZERO,
            speed: 1.42,
        }
    }
}

#[derive(Bundle, Default)]
pub struct MovementBundle {
    pub character_movement: CharacterMovement,
    pub control_movement: ControlMovement,
}
