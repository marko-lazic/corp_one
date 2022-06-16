use bevy::prelude::*;

use crate::constants::tick;

#[derive(Debug, Component)]
pub struct Movement {
    pub acceleration: f32,
    pub speed: f32,
    pub velocity: Vec3,
    pub can_move: bool,
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
            speed: 400.0,
            velocity: Vec3::ZERO,
            can_move: true,
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
        assert_eq!(movement.velocity, Vec3::new(4.7140455, 0.0, -4.7140455));
    }
}
