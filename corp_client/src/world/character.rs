use bevy::prelude::*;

const ONE_SECOND: f64 = 1.0;
const FRAME_RATE: f64 = 60.0;
const TIME_STEP: f32 = (ONE_SECOND / FRAME_RATE) as f32;

#[derive(Debug, Component)]
pub struct Movement {
    pub acceleration: f32,
    pub speed: f32,
    pub velocity: Vec3,
    pub direction: Vec3,
    pub can_move: bool,
    pub rotation_time: f32,
    pub rotating: bool,
    pub target_rotation: Quat,
    pub is_moving: bool,
}

impl Movement {
    pub fn update_direction(&mut self, direction: Vec3) {
        self.direction = direction;
    }
    pub fn update_velocity(&mut self) {
        self.velocity = self.direction * self.speed * TIME_STEP;
    }

    pub fn is_direction_zero(&self) -> bool {
        self.direction == Vec3::ZERO
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            acceleration: 10.0,
            speed: 400.0,
            velocity: Vec3::ZERO,
            direction: Vec3::ZERO,
            can_move: true,
            rotation_time: 0.0,
            rotating: false,
            target_rotation: Quat::IDENTITY,
            is_moving: false,
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
        movement.update_direction(direction);

        // when
        movement.update_velocity();

        // then
        assert_eq!(movement.velocity, Vec3::new(4.7140455, 0.0, -4.7140455));
    }
}
