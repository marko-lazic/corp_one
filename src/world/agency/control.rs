use bevy::{math::Vec3, prelude::info};
use std::ops::{AddAssign, SubAssign};

pub(crate) fn move_player(delta_move: &mut Vec3, action: &str) {
    if action == "MOVE_FORWARD" {
        delta_move.add_assign(Vec3::new(0.1, 0.0, 0.0));
    }
    if action == "MOVE_BACKWARD" {
        delta_move.add_assign(Vec3::new(-0.1, 0.0, 0.0));
    }
    if action == "MOVE_LEFT" {
        delta_move.add_assign(Vec3::new(0.0, 0.0, -0.1));
    }
    if action == "MOVE_RIGHT" {
        delta_move.add_assign(Vec3::new(0.0, 0.0, 0.1));
    }
}

pub(crate) fn aim_mouse(action: &str) {
    if action == "MOUSE_SHOOT" {
        info!("Bang");
    }
    if action == "AIM_UP" {}
    if action == "AIM_DOWN" {}
    if action == "AIM_LEFT" {}
    if action == "AIM_RIGHT" {}
}

pub(crate) fn rotate_camera(translation: &mut Vec3, action: &str) {
    let speed: f32 = 0.5;

    if action == "ARROW_LEFT" {
        translation.add_assign(Vec3::unit_x() * speed * 1.0);
    }
    if action == "ARROW_RIGHT" {
        translation.sub_assign(Vec3::unit_x() * speed * 1.0);
    }
}
