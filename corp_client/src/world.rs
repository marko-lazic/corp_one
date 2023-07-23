use bevy::prelude::*;

use crate::world::{
    animator::AnimatorPlugin,
    ccc::{CameraSet, CharacterPlugin, CharacterSet, ControlPlugin, ControlSet, MainCameraPlugin},
    colony::{zone::ZonePlugin, ColonyPlugin},
    physics::PhysicsPlugin,
    player::PlayerPlugin,
    star_map::StarMapPlugin,
    WorldSystemSet::{CameraSetup, PlayerSetup},
};

mod animator;
mod ccc;
mod cloning;
pub mod colony;
mod physics;
pub mod player;
mod star_map;

pub mod prelude {
    pub use super::ccc::{CharacterMovement, CursorWorld};
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldSystemSet {
    PlayerSetup,
    CameraSetup,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, CameraSetup.after(PlayerSetup))
            .insert_resource(AmbientLight {
                color: Color::ORANGE_RED,
                brightness: 0.8,
            })
            .add_plugins((
                PhysicsPlugin,
                ColonyPlugin,
                AnimatorPlugin,
                StarMapPlugin,
                CharacterPlugin,
                ControlPlugin,
                MainCameraPlugin,
                ZonePlugin,
                PlayerPlugin,
            ))
            .configure_set(
                Update,
                ControlSet::PlayingInput.before(CharacterSet::Movement),
            )
            .configure_set(Update, CameraSet::Update.after(CharacterSet::Movement));
    }
}
