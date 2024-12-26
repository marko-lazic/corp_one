use crate::{database::DbPlugin, server::ServerNetPlugin};
use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, state::app::StatesPlugin};
use std::time::Duration;

mod database;
mod dirs;
mod server;
mod table;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ServerState {
    #[default]
    Load,
    Serve,
}

fn main() {
    let frames_per_second = Duration::from_secs_f32(1.0 / 60.0);

    App::new()
        .add_plugins((
            LogPlugin::default(),
            StatesPlugin,
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frames_per_second)),
            DbPlugin,
            ServerNetPlugin,
        ))
        .init_state::<ServerState>()
        .run();
}
