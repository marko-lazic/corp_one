use bevy::prelude::*;

use gui::metrics::MetricsPlugin;

use crate::asset::loading::LoadingPlugin;
use crate::world::WorldPlugin;

mod asset;
mod audio;
mod gui;
mod world;

#[derive(Default)]
pub struct Game {
    _player_entity: Option<Entity>,
    camera_transform: Option<Transform>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    Loading,
    _StarMap,
    Playing,
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

mod options {
    pub const CORP_ONE_GAME_TITLE: &str = "Corp One";
    pub const WIDTH: f32 = 1600.0;
    pub const HEIGHT: f32 = 1600.0;
}

fn create_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: options::CORP_ONE_GAME_TITLE.to_string(),
        width: options::WIDTH,
        height: options::HEIGHT,
        ..Default::default()
    }
}
