use corp_types::AuthenticationEvent;
use tokio::sync::broadcast;
use tracing::warn;

#[derive(Clone)]
pub struct Events {
    auth_event_sender: broadcast::Sender<AuthenticationEvent>,
}

impl Events {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            auth_event_sender: sender,
        }
    }

    pub fn send(&self, event: AuthenticationEvent) {
        if let Err(e) = self.auth_event_sender.send(event) {
            warn!("Failed to send login event: {}", e);
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AuthenticationEvent> {
        self.auth_event_sender.subscribe()
    }
}
