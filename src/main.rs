use bevy::app::App;
use bevy::prelude::Msaa;
use bevy::AddDefaultPlugins;
use corp_console::ConsolePlugin;
use corp_metrics::MetricsPlugin;
use corp_scene::ScenePlugin;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(ConsolePlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(ScenePlugin)
        .run();
}
