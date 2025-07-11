use corp_types::prelude::*;
use kameo::prelude::ActorRef;
use kameo_actors::pubsub::{PubSub, Publish};
use tracing::error;

#[derive(Clone)]
pub struct Events {
    pub auth_event_subscribers: ActorRef<PubSub<AuthenticationEvent>>,
}

impl Events {
    pub fn new(auth_event_subscribers: ActorRef<PubSub<AuthenticationEvent>>) -> Self {
        Self {
            auth_event_subscribers,
        }
    }

    pub async fn send(&self, event: AuthenticationEvent) -> anyhow::Result<(), ApiError> {
        let result = self.auth_event_subscribers.ask(Publish(event)).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Error publishing authentication event: {:?}", err);
                Err(ApiError::internal_error())
            }
        }
    }
}
