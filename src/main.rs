use bevy::app::App;
use bevy::prelude::Msaa;
use bevy::DefaultPlugins;
use corp_console::ConsolePlugin;
use corp_input::InputPlugin;
use corp_metrics::MetricsPlugin;
use corp_scene::player::PlayerPlugin;
use corp_scene::ScenePlugin;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_startup_stage("game_setup")
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
