use bevy::prelude::*;

use audio::live::LivePlugin;
use gui::{console::ConsolePlugin, metrics::MetricsPlugin};
use world::{player::PlayerPlugin, scene::ScenePlugin};

use crate::loading::LoadingPlugin;
use crate::world::agency::input::InputPlugin;

mod audio;
mod gui;
mod loading;
mod paths;
mod world;

static CORP_ONE_GAME_TITLE: &str = "Corp One";

#[derive(Default)]
struct Game {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    Loading,
    _StarMap,
    Playing,
}

fn create_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: CORP_ONE_GAME_TITLE.to_string(),
        width: 1600.0,
        height: 1600.0,
        ..Default::default()
    }
}

fn main() {
    App::build()
        // .insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(create_window_descriptor())
        // .add_startup_stage(GAME_SETUP_STARTUP_STAGE)
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .init_resource::<Game>()
        .add_plugin(LoadingPlugin)
        .add_plugin(ConsolePlugin)
        .add_plugin(LivePlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
