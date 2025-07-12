use crate::server::*;
use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::*,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use corp_shared::{network::TICK_RATE, prelude::Colony};
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::{Infallible, PanicError},
    prelude::ActorStopReason,
    Actor,
};
use std::{ops::ControlFlow, time::Duration};
use tracing::{error, info};

pub struct GameServerActor {
    pub config: GameServerConfig,
}

impl GameServerActor {
    pub fn new(config: &GameServerConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Actor for GameServerActor {
    type Args = Self;
    type Error = Infallible;

    async fn on_start(args: Self::Args, _actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        info!("GameServerActor started with config: {:?}", args.config);

        let config = args.config.clone();
        let game_server_handle = std::thread::Builder::new()
            .name(format!(
                "game-server-{}",
                config.colony.to_string().to_lowercase()
            ))
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                if config.colony == Colony::StarMap {
                    create_star_map_game_server(config);
                } else {
                    create_colony_game_server(config);
                }
            });

        if let Err(e) = game_server_handle {
            error!("Failed to start game server thread: {:?}", e);
        }

        Ok(args)
    }

    async fn on_panic(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        err: PanicError,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        error!(
            "GameServerActor: Game Server {:?} panicked: {}",
            self.config.colony, err
        );
        Ok(ControlFlow::Continue(()))
    }

    async fn on_stop(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        reason: ActorStopReason,
    ) -> Result<(), Self::Error> {
        info!(
            "GameServerActor: Game Server {:?} stopping: {:?}",
            self.config.colony, reason
        );
        Ok(())
    }
}

fn create_colony_game_server(game_server_config: GameServerConfig) {
    let wait_duration = Duration::from_secs_f64(1.0 / f64::from(TICK_RATE));

    App::new()
        .insert_resource(game_server_config)
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(wait_duration)),
            StatesPlugin,
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

fn create_star_map_game_server(game_server_config: GameServerConfig) {
    let wait_duration = Duration::from_secs_f64(1.0 / f64::from(TICK_RATE));

    App::new()
        .insert_resource(game_server_config)
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(wait_duration)),
            StatesPlugin,
            ServerNetPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .run();
}
