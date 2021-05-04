use bevy::prelude::*;

use gui::metrics::MetricsPlugin;

use crate::loading::LoadingPlugin;
use crate::world::WorldPlugin;

mod audio;
mod gui;
mod loading;
mod paths;
mod world;

mod options {
    pub const CORP_ONE_GAME_TITLE: &str = "Corp One";
    pub const WIDTH: f32 = 1600.0;
    pub const HEIGHT: f32 = 1600.0;
}

#[derive(Default)]
struct Game {
    _player_entity: Option<Entity>,
    camera_transform: Option<Transform>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    Loading,
    _StarMap,
    Playing,
}

fn create_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: options::CORP_ONE_GAME_TITLE.to_string(),
        width: options::WIDTH,
        height: options::HEIGHT,
        ..Default::default()
    }
}

fn main() {
    App::build()
        // .insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(create_window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .init_resource::<Game>()
        .add_plugin(LoadingPlugin)
        // .add_plugin(ConsolePlugin)
        // .add_plugin(LivePlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(WorldPlugin)
        .run();
}
