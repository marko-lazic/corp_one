use bevy::prelude::*;

use crate::world::animator::AnimatorPlugin;
use crate::world::ccc::camera::{CameraSet, MainCameraPlugin};
use crate::world::ccc::character::{CharacterPlugin, CharacterSet};
use crate::world::ccc::control::{ControlPlugin, ControlSet};
use crate::world::colony::zone::ZonePlugin;
use crate::world::colony::ColonyPlugin;
use crate::world::physics::PhysicsPlugin;
use crate::world::player::PlayerPlugin;
use crate::world::star_map::StarMapPlugin;
use crate::world::WorldSystemSet::{CameraSetup, PlayerSetup};

mod animator;
mod ccc;
mod cloning;
pub mod colony;
mod physics;
pub mod player;
mod star_map;

pub mod prelude {
    pub use super::ccc::control::CursorWorld;
    pub use super::ccc::movement::CharacterMovement;
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldSystemSet {
    PlayerSetup,
    CameraSetup,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(CameraSetup.after(PlayerSetup))
            .insert_resource(AmbientLight {
                color: Color::ORANGE_RED,
                brightness: 0.8,
            })
            .add_plugin(PhysicsPlugin)
            .add_plugin(ColonyPlugin)
            .add_plugin(AnimatorPlugin)
            .add_plugin(StarMapPlugin)
            .add_plugin(CharacterPlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(MainCameraPlugin)
            .add_plugin(ZonePlugin)
            .add_plugin(PlayerPlugin)
            .configure_set(ControlSet::Input.before(CharacterSet::Movement))
            .configure_set(CameraSet::Update.after(CharacterSet::Movement));
    }
}
