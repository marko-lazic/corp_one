use crate::PlayerRepository;
use anyhow::Result;
use corp_types::prelude::*;
use futures_lite::StreamExt;
use kameo::{Actor, actor::ActorRef, error::Infallible};
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct PlayerServiceActor {
    player_repository: PlayerRepository,
    messenger: Messenger,
}

impl PlayerServiceActor {
    pub fn new(repository: &PlayerRepository, message: &Messenger) -> Self {
        Self {
            player_repository: repository.clone(),
            messenger: message.clone(),
        }
    }

    pub async fn start_consuming(&self) -> Result<()> {
        info!("Starting JetStream message consumption for PlayerService");

        let stream = self.messenger.js.get_stream("USER_EVENTS").await?;

        let consumer = stream
            .get_or_create_consumer(
                "world-user-registered",
                async_nats::jetstream::consumer::pull::Config {
                    durable_name: Some("world-user-registered".to_string()),
                    filter_subject: "user.registered".to_string(),
                    deliver_policy: async_nats::jetstream::consumer::DeliverPolicy::All,
                    ack_policy: async_nats::jetstream::consumer::AckPolicy::Explicit,
                    max_deliver: 3,
                    ..Default::default()
                },
            )
            .await?;

        // Subscribe to messages
        let mut messages = consumer.messages().await?;

        while let Some(message) = messages.next().await {
            match message {
                Ok(msg) => match self.handle_nats_message(msg.message.clone()).await {
                    Ok(_) => {
                        if let Err(e) = msg.ack().await {
                            error!("Failed to acknowledge message: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to handle JetStream message: {}", e);
                        if let Err(ack_e) = msg.double_ack().await {
                            error!("Failed to nack message: {}", ack_e);
                        }
                    }
                },
                Err(e) => {
                    error!("Error receiving JetStream message: {}", e);
                }
            }
        }

        info!("JetStream subscription ended");
        Ok(())
    }

    async fn handle_nats_message(&self, message: async_nats::Message) -> Result<()> {
        let user_event: UserEvent = rmp_serde::from_slice(&message.payload)?;

        match user_event {
            UserEvent::Register(user) => {
                info!("Received Register event for user: {}", user.username);
                self.handle_user_register(user).await?;
            }
            _ => {
                // Ignore other user events
            }
        }

        Ok(())
    }

    async fn handle_user_register(&self, user: User) -> Result<()> {
        let username = &user.username;

        // Check if player already exists
        if self.player_repository.player_exists(username).await? {
            warn!("Player already exists for user: {}", username);
            return Ok(());
        }

        // Create new player with default values
        let character_name = format!("{}_character", username);
        let faction = self.assign_default_faction(username);

        let player = Player::new(username.clone(), character_name, faction);

        match self.player_repository.create_player(player).await {
            Ok(created_player) => {
                info!(
                    "Created player for user: {} with character: {}",
                    username, created_player.character_name
                );
            }
            Err(e) => {
                error!("Failed to create player for user {}: {}", username, e);
                return Err(e);
            }
        }

        Ok(())
    }

    fn assign_default_faction(&self, username: &str) -> Faction {
        // Simple hash-based faction assignment for demo
        let hash = username.chars().map(|c| c as u32).sum::<u32>();
        match hash % 3 {
            0 => Faction::EC,
            1 => Faction::CMG,
            _ => Faction::VI,
        }
    }
}

impl Actor for PlayerServiceActor {
    type Args = Self;
    type Error = Infallible;

    async fn on_start(state: Self::Args, _actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        // Start NATS consumption in background task
        let player_service_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = player_service_clone.start_consuming().await {
                error!("PlayerService NATS consumption failed: {}", e);
            }
        });

        Ok(state)
    }
}

impl std::fmt::Debug for PlayerServiceActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerActor").finish()
    }
}
