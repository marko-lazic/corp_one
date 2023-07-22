use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

fn main() {
    let frames_per_second = Duration::from_secs_f64(1.0 / 60.0);
    App::new()
        .add_plugins((
            MinimalPlugins,
            ScheduleRunnerPlugin::run_loop(frames_per_second),
        ))
        .run();
}
