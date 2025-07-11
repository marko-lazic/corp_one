use crate::event::Events;
use anyhow::Result;
use corp_login::run_server;
use corp_types::prelude::*;
use kameo::actor::ActorRef;
use kameo_actors::pubsub::PubSub;
use tracing_subscriber::fmt::init;

mod event;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let actor_ref = ActorRef::<PubSub<AuthenticationEvent>>::lookup("auth_event_bus")?.unwrap();
    let events = Events::new(actor_ref);
    run_server(events).await?;
    Ok(())
}
