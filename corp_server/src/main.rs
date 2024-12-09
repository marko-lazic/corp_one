use std::time::Duration;

use crate::database::DbPlugin;
use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, state::app::StatesPlugin};

mod database;
mod dirs;
mod table;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ServerState {
    #[default]
    Load,
    Serve,
}

fn main() {
    let frames_per_second = Duration::from_secs_f64(1.0 / 60.0);

    App::new()
        .add_plugins((
            LogPlugin::default(),
            StatesPlugin,
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frames_per_second)),
            DbPlugin,
        ))
        .init_state::<ServerState>()
        .run();
}
