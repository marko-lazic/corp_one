use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::{Infallible, PanicError},
    prelude::{ActorID, ActorStopReason},
    Actor,
};
use std::ops::ControlFlow;
use tracing::info;

pub struct ProxyActor;

impl Actor for ProxyActor {
    type Args = Self;
    type Error = Infallible;

    async fn on_start(args: Self::Args, _actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        info!("Proxy Actor started");
        tokio::spawn(async move {
            let _ = corp_proxy::init::init().await;
        });
        Ok(args)
    }

    async fn on_stop(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        reason: ActorStopReason,
    ) -> Result<(), Self::Error> {
        info!("Proxy Actor stopping: {:?}", reason);
        Ok(())
    }

    async fn on_panic(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        err: PanicError,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        tracing::error!("Proxy Actor panicked: {}", err);
        Ok(ControlFlow::Continue(()))
    }
}
