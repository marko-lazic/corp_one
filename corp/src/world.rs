use bevy::prelude::*;

pub mod agency;
pub mod camera;
pub mod cube;
pub mod player;
pub mod scene;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum WorldSystem {
    PlayerSetup,
    CameraSetup,
}
