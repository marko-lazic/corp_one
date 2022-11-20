use std::time::Duration;

use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;

fn main() {
    let frames_per_second = Duration::from_secs_f64(1.0 / 60.0);
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(frames_per_second))
        .add_plugins(MinimalPlugins)
        .run();
}
