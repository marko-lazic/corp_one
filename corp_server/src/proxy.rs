use kameo::{actor::ActorRef, error::Infallible, Actor};
use tracing::info;

pub struct ProxyActor;

impl Actor for ProxyActor {
    type Error = Infallible;

    async fn on_start(&mut self, _actor_ref: ActorRef<Self>) -> Result<(), Self::Error> {
        info!("Proxy Actor started");
        let _ = corp_proxy::init::init().await;
        Ok(())
    }
}
