use bevy::prelude::*;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::camera::TopDownCameraPlugin;
use crate::world::cursor::MyRaycastSet;
use crate::world::input_command::PlayerCommand;
use crate::world::input_control::InputControlPlugin;
use crate::world::player::PlayerPlugin;
use crate::world::scene::ScenePlugin;
use bevy::core::FixedTimestep;
use bevy_mod_raycast::DefaultRaycastingPlugin;

pub mod camera;
pub mod character;
mod cursor;
pub mod flying_cubes;
pub mod input_command;
pub mod input_control;
pub mod player;
mod player_bundle;
pub mod scene;
mod world_utils;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum WorldSystem {
    SetupPlayer,
    SetupCamera,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerCommand>();
        app.add_plugin(ScenePlugin);
        app.add_plugin(InputControlPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(TopDownCameraPlugin);
        app.add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default());
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(cursor::update_raycast_with_cursor.system()),
        );
    }
}
