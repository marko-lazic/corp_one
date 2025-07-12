use corp_login::Events;
use corp_types::prelude::*;
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::{Infallible, PanicError},
    prelude::ActorStopReason,
    Actor,
};
use kameo_actors::pubsub::PubSub;
use std::ops::ControlFlow;
use tracing::info;

pub struct LoginActor {
    auth_sub_ref: ActorRef<PubSub<AuthenticationEvent>>,
}

impl LoginActor {
    pub fn new(auth_sub_reg: ActorRef<PubSub<AuthenticationEvent>>) -> LoginActor {
        Self {
            auth_sub_ref: auth_sub_reg,
        }
    }
}

impl Actor for LoginActor {
    type Args = Self;
    type Error = Infallible;

    async fn on_start(args: Self::Args, _actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        let events = Events::new(args.auth_sub_ref.clone());
        info!("LoginActor started");
        tokio::spawn(async move {
            let _ = corp_login::run_server(events).await;
        });
        Ok(args)
    }

    async fn on_panic(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        err: PanicError,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        tracing::error!("LoginActor panicked: {}", err);
        Ok(ControlFlow::Continue(()))
    }

    async fn on_stop(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        reason: ActorStopReason,
    ) -> Result<(), Self::Error> {
        info!("LoginActor stopping: {:?}", reason);
        Ok(())
    }
}
