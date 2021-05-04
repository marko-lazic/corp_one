use bevy::prelude::*;

pub mod camera;
pub mod character;
pub mod control;
pub mod cube;
pub mod player;
pub mod scene;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum WorldSystem {
    PlayerSetup,
    CameraSetup,
}
