use std::time::Duration;

use bevy::app::ScheduleRunnerSettings;
use bevy::log::LogPlugin;
use bevy::prelude::*;

use crate::server::ServerPlugin;

mod server;

fn main() {
    let frames_per_second = Duration::from_secs_f64(1.0 / 60.0);
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(frames_per_second))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ServerPlugin)
        .run();
}