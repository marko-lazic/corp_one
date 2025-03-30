use crate::game::{app::new_app, config::ServerConfig};
use kameo::{actor::ActorRef, error::Infallible, Actor};

pub struct GameInstanceActor {
    pub config: ServerConfig,
}

impl Actor for GameInstanceActor {
    type Error = Infallible;

    async fn on_start(&mut self, _actor_ref: ActorRef<Self>) -> Result<(), Self::Error> {
        println!("Actor started with config: {:?}", self.config);
        new_app(self.config);
        Ok(())
    }
}
