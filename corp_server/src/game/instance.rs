use crate::game::*;
use kameo::{actor::ActorRef, error::Infallible, Actor};
use tracing::info;

pub struct GameInstanceActor {
    pub config: GameInstanceConfig,
}

impl Actor for GameInstanceActor {
    type Error = Infallible;

    async fn on_start(&mut self, _actor_ref: ActorRef<Self>) -> Result<(), Self::Error> {
        info!("Actor started with config: {:?}", self.config);
        new_app(self.config.clone());
        Ok(())
    }
}
