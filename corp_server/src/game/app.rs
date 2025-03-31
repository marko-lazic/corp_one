use crate::game::*;
use bevy::{app::ScheduleRunnerPlugin, prelude::*, state::app::StatesPlugin};
use bevy_rand::prelude::*;
use corp_shared::prelude::*;
use std::time::Duration;

pub fn new_app(instance_config: GameInstanceConfig) {
    let wait_duration = Duration::from_secs_f64(1.0 / f64::from(TICK_RATE));

    App::new()
        .insert_resource(instance_config)
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(wait_duration)),
            StatesPlugin,
            DbPlugin,
            ServerNetPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .run();
}
