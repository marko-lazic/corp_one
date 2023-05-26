use bevy::prelude::*;

use crate::input::InputControlPlugin;
use crate::world::animator::AnimatorPlugin;
use crate::world::camera::TopDownCameraPlugin;
use crate::world::colony::zone::ZonePlugin;
use crate::world::colony::ColonyPlugin;
use crate::world::physics::PhysicsPlugin;
use crate::world::player::PlayerPlugin;
use crate::world::star_map::StarMapPlugin;
use crate::world::WorldSystemSet::{CameraSetup, PlayerSetup};

mod animator;
pub mod camera;
pub mod character;
mod cloning;
pub mod colony;
mod physics;
pub mod player;
mod star_map;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldSystemSet {
    PlayerSetup,
    CameraSetup,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(CameraSetup.after(PlayerSetup));
        app.insert_resource(AmbientLight {
            color: Color::ORANGE_RED,
            brightness: 0.8,
        });
        app.add_plugin(PhysicsPlugin);
        app.add_plugin(ColonyPlugin);
        app.add_plugin(AnimatorPlugin);
        app.add_plugin(StarMapPlugin);
        app.add_plugin(InputControlPlugin);
        app.add_plugin(ZonePlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(TopDownCameraPlugin);
    }
}
