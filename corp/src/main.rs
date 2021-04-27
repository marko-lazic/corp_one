use bevy::prelude::*;

use audio::live::LivePlugin;
use gui::{console::ConsolePlugin, metrics::MetricsPlugin};
use world::{player::PlayerPlugin, scene::ScenePlugin};

use crate::loading::LoadingPlugin;
use crate::world::agency::input::InputPlugin;
use crate::world::camera::TopDownCameraPlugin;

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
    player: Option<Entity>,
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
        .add_plugin(ConsolePlugin)
        .add_plugin(LivePlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TopDownCameraPlugin)
        .run();
}
