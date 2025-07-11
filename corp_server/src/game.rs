use crate::{
    login::{GameServerRegistered, LoginActor},
    server::*,
};
use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::PluginGroup,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use corp_shared::{network::TICK_RATE, prelude::Colony};
use corp_types::AuthenticationEvent;
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::{Infallible, PanicError},
    prelude::{ActorID, ActorStopReason, Context, Message},
    Actor,
};
use std::{ops::ControlFlow, time::Duration};
use tracing::info;

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

    async fn on_start(args: Self::Args, actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        info!("GameServerActor started with config: {:?}", args.config);

        // Self-register with LoginActor
        let server_name = format!("{:?}", args.config.colony).to_lowercase();
        match ActorRef::<LoginActor>::lookup("login") {
            Ok(Some(login_ref)) => {
                let registration = GameServerRegistered {
                    name: server_name.clone(),
                    actor_ref,
                };
                if let Err(e) = login_ref.tell(registration).await {
                    tracing::warn!(
                        "GameServerActor: Failed to register {} with LoginActor: {}",
                        server_name,
                        e
                    );
                } else {
                    info!(
                        "GameServerActor: Successfully registered {} with LoginActor",
                        server_name
                    );
                }
            }
            Ok(None) => {
                tracing::warn!(
                    "GameServerActor: LoginActor not found in registry, {} cannot self-register",
                    server_name
                );
            }
            Err(e) => {
                tracing::warn!(
                    "GameServerActor: Registry lookup error for LoginActor: {}",
                    e
                );
            }
        }

        let config = args.config.clone();
        tokio::spawn(async move {
            if config.colony == Colony::StarMap {
                create_star_map_game_server(config);
            } else {
                create_colony_game_server(config);
            }
        });

        Ok(args)
    }

    async fn on_panic(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        err: PanicError,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        tracing::error!(
            "GameServerActor: Game Server {:?} panicked: {}",
            self.config.colony,
            err
        );
        Ok(ControlFlow::Continue(()))
    }

    async fn on_link_died(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        id: ActorID,
        reason: ActorStopReason,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        match reason {
            ActorStopReason::Normal => {
                info!(
                    "GameServerActor: Game Server {:?} - linked actor {} stopped normally",
                    self.config.colony, id
                );
                Ok(ControlFlow::Continue(()))
            }
            _ => {
                tracing::warn!(
                    "GameServerActor: Game Server {:?} - linked actor {} died: {:?}",
                    self.config.colony,
                    id,
                    reason
                );
                Ok(ControlFlow::Continue(()))
            }
        }
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

impl Message<AuthenticationEvent> for GameServerActor {
    type Reply = ();

    async fn handle(&mut self, msg: AuthenticationEvent, _ctx: &mut Context<Self, Self::Reply>) {
        let colony = self.config.colony;
        info!(
            "GameServerActor: Game Server {:?} received AuthenticationEvent: {:?}",
            colony, msg.event_type
        );
        // Add actual authentication event processing here
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
