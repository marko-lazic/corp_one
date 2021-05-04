use crate::world::camera::TopDownCameraPlugin;
use crate::world::command::PlayerCommand;
use crate::world::control::ControlPlugin;
use crate::world::player::PlayerPlugin;
use crate::world::scene::ScenePlugin;
use bevy::prelude::*;

pub mod camera;
pub mod character;
pub mod command;
pub mod control;
pub mod cube;
pub mod player;
mod player_bundle;
pub mod scene;

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
        app.add_plugin(ControlPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(TopDownCameraPlugin);
    }
}
