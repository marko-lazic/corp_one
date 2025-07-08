use crate::server::{
    cloning::CloningRemotePlugin, death::DeathPlugin, health::HealthRemotePlugin, *,
};
use bevy::{app::ScheduleRunnerPlugin, prelude::*, state::app::StatesPlugin};
use bevy_rand::prelude::*;
use corp_shared::prelude::*;
use std::time::Duration;

pub fn colony_app(instance_config: ColonyAppConfig) {
    let wait_duration = Duration::from_secs_f64(1.0 / f64::from(TICK_RATE));

    App::new()
        .insert_resource(instance_config)
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(wait_duration)),
            StatesPlugin,
            DbPlugin,
            ServerNetPlugin,
            LootPlugin,
            HealthRemotePlugin,
            DeathPlugin,
            CloningRemotePlugin,
            PlayersPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .run();
}

pub fn star_map_app(instance_config: ColonyAppConfig) {
    let wait_duration = Duration::from_secs_f64(1.0 / f64::from(TICK_RATE));

    App::new()
        .insert_resource(instance_config)
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(wait_duration)),
            StatesPlugin,
            ServerNetPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .run();
}
