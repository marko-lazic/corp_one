use bevy::app::App;
use bevy::AddDefaultPlugins;
use corp_console::ConsolePlugin;
use corp_metrics::MetricsPlugin;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(ConsolePlugin)
        .add_plugin(MetricsPlugin)
        .run();
}
