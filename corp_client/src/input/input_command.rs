use bevy::math::Vec3;
use bevy::prelude::Resource;
use leafwing_input_manager::action_state::ActionState;

use crate::input::CorpAction;

#[derive(Resource, Default)]
pub struct PlayerDirection {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerDirection {
    pub fn move_action(&mut self, action_state: &ActionState<CorpAction>) {
        if action_state.pressed(CorpAction::Forward) {
            self.forward = true;
        }
        if action_state.pressed(CorpAction::Backward) {
            self.backward = true;
        }
        if action_state.pressed(CorpAction::Left) {
            self.left = true;
        }
        if action_state.pressed(CorpAction::Right) {
            self.right = true;
        }
    }

    pub fn shoot_action(&mut self, action_state: &ActionState<CorpAction>) {
        if action_state.just_pressed(CorpAction::Shoot) {
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
    pub fn new_direction(&self) -> Vec3 {
        let mut direction = Vec3::ZERO;
        if self.forward {
            direction += Vec3::Z;
        }
        if self.backward {
            direction -= Vec3::Z;
        }
        if self.left {
            direction += Vec3::X;
        }
        if self.right {
            direction -= Vec3::X;
        }
        direction.normalize_or_zero()
    }
}
