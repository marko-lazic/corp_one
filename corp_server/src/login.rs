use kameo::{actor::ActorRef, error::Infallible, Actor};
use tracing::info;

pub struct LoginActor;

impl Actor for LoginActor {
    type Error = Infallible;

    async fn on_start(&mut self, _actor_ref: ActorRef<Self>) -> Result<(), Self::Error> {
        info!("Login Actor started");
        let _ = corp_login::run_server().await;
        Ok(())
    }
}
