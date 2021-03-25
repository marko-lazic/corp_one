mod gui;
mod world;
mod audio;

//use audio::live::LivePlugin;
use bevy::app::App;
use bevy::prelude::Msaa;
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use gui::{console::ConsolePlugin, metrics::MetricsPlugin};
use world::{agency::input::InputPlugin, player::PlayerPlugin, scene::ScenePlugin};

pub static GAME_SETUP_STARTUP_STAGE: &str = "game_setup";
static CORP_ONE_GAME_TITLE: &str = "Corp One";

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: CORP_ONE_GAME_TITLE.to_string(),
            width: 1600.0,
            height: 1600.0,
            ..Default::default()
        })
        // .add_startup_stage(GAME_SETUP_STARTUP_STAGE)
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        // .add_plugin(LivePlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
