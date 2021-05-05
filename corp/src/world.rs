use bevy::prelude::*;

use crate::world::camera::TopDownCameraPlugin;
use crate::world::input_command::PlayerCommand;
use crate::world::input_control::InputControlPlugin;
use crate::world::player::PlayerPlugin;
use crate::world::scene::ScenePlugin;

pub mod camera;
pub mod character;
pub mod flying_cubes;
pub mod input_command;
pub mod input_control;
pub mod player;
mod player_bundle;
pub mod scene;
mod world_utils;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum WorldSystem {
    PlayerSetup,
    CameraSetup,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerCommand>();
        app.add_plugin(ScenePlugin);
        app.add_plugin(InputControlPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(TopDownCameraPlugin);
    }
}
