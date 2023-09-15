use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};

use crate::database::DbPlugin;

mod database;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ServerState {
    #[default]
    Load,
    Serve,
}

fn main() {
    let frames_per_second = Duration::from_secs_f64(1.0 / 60.0);

    App::new()
        .add_state::<ServerState>()
        .add_plugins((
            LogPlugin::default(),
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frames_per_second)),
            bevy_tokio_tasks::TokioTasksPlugin::default(),
            DbPlugin,
        ))
        .run();
}
