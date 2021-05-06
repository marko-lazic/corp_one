use bevy::math::Vec3;
use bevy::prelude::Transform;

#[derive(Default)]
pub struct PlayerCommand {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerCommand {
    pub fn key_command(&mut self, action: &str) {
        if action == "MOVE_FORWARD" {
            self.forward = true;
        }
        if action == "MOVE_BACKWARD" {
            self.backward = true;
        }
        if action == "MOVE_LEFT" {
            self.left = true;
        }
        if action == "MOVE_RIGHT" {
            self.right = true;
        }
    }

    pub fn mouse_command(&mut self, action: &str) {
        if action == "MOUSE_SHOOT" {
            bevy::log::info!("Bang");
        }
        if action == "AIM_UP" {}
        if action == "AIM_DOWN" {}
        if action == "AIM_LEFT" {}
        if action == "AIM_RIGHT" {}
    }

    pub fn reset(&mut self) {
        self.forward = false;
        self.backward = false;
        self.left = false;
        self.right = false;
    }

    /// X is sides
    /// Y is up/down
    /// Z is front/back
    pub fn new_direction(&self, position: &Transform) -> Vec3 {
        let mut direction = Vec3::ZERO;
        if self.forward {
            direction -= position.local_z();
        }
        if self.backward {
            direction += position.local_z();
        }
        if self.left {
            direction -= position.local_x();
        }
        if self.right {
            direction += position.local_x();
        }
        direction = direction.normalize_or_zero();
        direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_direction() {
        // given
        let mut command = PlayerCommand::default();
        command.key_command("MOVE_RIGHT");
        command.key_command("MOVE_FORWARD");
        let position = Transform::from_xyz(0.0, 0.0, 0.0);

        // when
        let direction = command.new_direction(&position);

        let expected = std::f32::consts::FRAC_1_SQRT_2;

        // then
        assert_eq!(direction, Vec3::new(expected, 0.0, -expected));
    }
}
