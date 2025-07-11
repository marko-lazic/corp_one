use crate::{
    login::{GameServerRegistered, LoginActor},
    server::{colony_app, star_map_app, ColonyAppConfig},
};
use corp_shared::prelude::Colony;
use corp_types::AuthenticationEvent;
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::{Infallible, PanicError},
    prelude::{ActorID, ActorStopReason, Context, Message},
    Actor,
};
use std::ops::ControlFlow;
use tracing::info;

pub struct GameServerActor {
    pub config: ColonyAppConfig,
}

impl Actor for GameServerActor {
    type Args = Self;
    type Error = Infallible;

    async fn on_start(args: Self::Args, actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        info!("Game Server Actor started with config: {:?}", args.config);

        // Self-register with LoginActor
        let server_name = format!("{:?}", args.config.colony).to_lowercase();
        match ActorRef::<LoginActor>::lookup("login") {
            Ok(Some(login_ref)) => {
                let registration = GameServerRegistered {
                    name: server_name.clone(),
                    actor_ref,
                };
                if let Err(e) = login_ref.tell(registration).await {
                    tracing::warn!("Failed to register {} with LoginActor: {}", server_name, e);
                } else {
                    info!("Successfully registered {} with LoginActor", server_name);
                }
            }
            Ok(None) => {
                tracing::warn!(
                    "LoginActor not found in registry, {} cannot self-register",
                    server_name
                );
            }
            Err(e) => {
                tracing::warn!("Registry lookup error for LoginActor: {}", e);
            }
        }

        let config = args.config.clone();
        tokio::spawn(async move {
            if config.colony == Colony::StarMap {
                star_map_app(config);
            } else {
                colony_app(config);
            }
        });

        Ok(args)
    }

    async fn on_panic(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        err: PanicError,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        tracing::error!("Game Server {:?} panicked: {}", self.config.colony, err);
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
                    "Game Server {:?} - linked actor {} stopped normally",
                    self.config.colony, id
                );
                Ok(ControlFlow::Continue(()))
            }
            _ => {
                tracing::warn!(
                    "Game Server {:?} - linked actor {} died: {:?}",
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
            "Game Server {:?} stopping: {:?}",
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
            "Game Server {:?} received AuthenticationEvent: {:?}",
            colony, msg.event_type
        );
        // Add actual authentication event processing here
    }
}
