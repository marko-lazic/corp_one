use crate::game::GameServerActor;
use corp_types::AuthenticationEvent;
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::{Infallible, PanicError},
    message::Context,
    prelude::{ActorID, ActorStopReason, Message},
    Actor,
};
use std::{collections::HashMap, ops::ControlFlow};
use tracing::info;

pub struct LoginActor {
    servers: HashMap<String, ActorRef<GameServerActor>>,
}

impl LoginActor {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }
}

pub struct GameServerRegistered {
    pub name: String,
    pub actor_ref: ActorRef<GameServerActor>,
}

pub struct GameServerUnregistered {
    pub name: String,
    pub actor_ref: ActorRef<GameServerActor>,
}

impl Message<GameServerRegistered> for LoginActor {
    type Reply = ();

    async fn handle(&mut self, msg: GameServerRegistered, _ctx: &mut Context<Self, Self::Reply>) {
        info!("LoginActor: Game server '{}' registered", msg.name);
        self.servers.insert(msg.name, msg.actor_ref);
    }
}

impl Message<GameServerUnregistered> for LoginActor {
    type Reply = ();

    async fn handle(&mut self, msg: GameServerUnregistered, _ctx: &mut Context<Self, Self::Reply>) {
        if let Some(_) = self.servers.remove(&msg.name) {
            info!("LoginActor: Game server '{}' unregistered", msg.name);
        }
    }
}

impl Actor for LoginActor {
    type Args = Self;
    type Error = Infallible;

    async fn on_start(args: Self::Args, actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        info!("LoginActor started");
        let events = corp_login::Events::new();

        let actor_ref_clone = actor_ref.clone();
        let events_clone = events.clone();
        tokio::spawn(async move {
            while let Ok(auth_event) = events_clone.subscribe().recv().await {
                if let Err(e) = actor_ref_clone.tell(auth_event).await {
                    tracing::warn!("Failed to send auth event to LoginActor: {}", e);
                }
            }
        });

        tokio::spawn(async move {
            let _ = corp_login::run_server(&events).await;
        });

        Ok(args)
    }

    async fn on_panic(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        err: PanicError,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        tracing::error!("Login Actor panicked: {}", err);
        Ok(ControlFlow::Continue(()))
    }

    async fn on_link_died(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        id: ActorID,
        reason: ActorStopReason,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        // Find and remove the dead game server from our registry
        let mut server_to_remove = None;
        for (name, actor_ref) in &self.servers {
            if actor_ref.id() == id {
                server_to_remove = Some(name.clone());
                break;
            }
        }

        if let Some(server_name) = server_to_remove {
            self.servers.remove(&server_name);
            match reason {
                ActorStopReason::Normal => {
                    info!(
                        "LoginActor: Game server '{}' stopped normally and removed from registry",
                        server_name
                    );
                }
                _ => {
                    tracing::warn!(
                        "LoginActor: Game server '{}' died ({:?}) and removed from registry",
                        server_name,
                        reason
                    );
                }
            }
        } else {
            match reason {
                ActorStopReason::Normal => {
                    info!("Linked actor {} stopped normally", id);
                }
                _ => {
                    tracing::warn!("Linked actor {} died: {:?}", id, reason);
                }
            }
        }

        Ok(ControlFlow::Continue(()))
    }

    async fn on_stop(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        reason: ActorStopReason,
    ) -> Result<(), Self::Error> {
        info!("Login Actor stopping: {:?}", reason);
        Ok(())
    }
}

impl Message<AuthenticationEvent> for LoginActor {
    type Reply = ();

    async fn handle(
        &mut self,
        auth_event: AuthenticationEvent,
        _ctx: &mut Context<Self, Self::Reply>,
    ) {
        info!("Received auth event: {:?}", auth_event.event_type);
        for (server_name, game_server_actor_ref) in &self.servers {
            if let Err(e) = game_server_actor_ref.tell(auth_event.clone()).await {
                tracing::warn!("Failed to send auth event to {}: {}", server_name, e);
            }
        }
    }
}
