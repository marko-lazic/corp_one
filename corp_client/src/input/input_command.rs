use bevy::math::Vec3;
use bevy::prelude::Transform;
use bevy_input_actionmap::InputMap;

use crate::input::Action;
use crate::{Input, MouseButton, Res};

#[derive(Default)]
pub struct PlayerAction {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerAction {
    pub fn key_action(&mut self, input: &Res<InputMap<Action>>) {
        if input.active(Action::Forward) {
            self.forward = true;
        }
        if input.active(Action::Backward) {
            self.backward = true;
        }
        if input.active(Action::Left) {
            self.left = true;
        }
        if input.active(Action::Right) {
            self.right = true;
        }
    }

    pub fn mouse_action(&mut self, buttons: &Res<Input<MouseButton>>) {
        if buttons.just_pressed(MouseButton::Left) {
            bevy::log::info!("Bang");
        }
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
            direction += position.local_z();
        }
        if self.backward {
            direction -= position.local_z();
        }
        if self.left {
            direction += position.local_x();
        }
        if self.right {
            direction -= position.local_x();
        }
        direction = direction.normalize_or_zero();
        direction
    }
}
