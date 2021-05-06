use bevy::prelude::*;

use constants::state::GameState;
use constants::window;
use gui::metrics::MetricsPlugin;

use crate::asset::loading::LoadingPlugin;
use crate::world::WorldPlugin;

mod asset;
mod audio;
mod constants;
mod gui;
mod world;

#[derive(Default)]
pub struct Game {
    _player_entity: Option<Entity>,
    camera_transform: Option<Transform>,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(create_window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .init_resource::<Game>()
        .add_plugin(LoadingPlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(WorldPlugin)
        .run();
}

fn create_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: window::CORP_ONE_GAME_TITLE.to_string(),
        width: window::WIDTH,
        height: window::HEIGHT,
        ..Default::default()
    }
}
