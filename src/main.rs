use bevy::app::App;
use bevy::prelude::Msaa;
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use corp_console::ConsolePlugin;
use corp_input::InputPlugin;
use corp_metrics::MetricsPlugin;
use corp_scene::player::PlayerPlugin;
use corp_scene::ScenePlugin;

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
