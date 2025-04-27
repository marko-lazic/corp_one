use crate::game::*;
use kameo::{actor::ActorRef, error::Infallible, Actor};
use tracing::info;

pub struct ColonyAppActor {
    pub config: ColonyAppConfig,
}

impl Actor for ColonyAppActor {
    type Error = Infallible;

    async fn on_start(&mut self, _actor_ref: ActorRef<Self>) -> Result<(), Self::Error> {
        info!("Actor started with config: {:?}", self.config);
        colony_app(self.config.clone());
        Ok(())
    }
}

pub struct StarMapAppActor {
    pub config: ColonyAppConfig,
}

impl Actor for StarMapAppActor {
    type Error = Infallible;

    async fn on_start(&mut self, _actor_ref: ActorRef<Self>) -> Result<(), Self::Error> {
        info!("Actor started with config: {:?}", self.config);
        star_map_app(self.config.clone());
        Ok(())
    }
}
