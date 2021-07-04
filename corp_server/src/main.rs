use bevy::app::ScheduleRunnerSettings;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_system(hello_world.system())
        .run();
}

fn hello_world() {
    info!("Hello world!");
}
