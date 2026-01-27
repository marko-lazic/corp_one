use crate::{PlayerRepository, SessionRepository};
use anyhow::Result;
use corp_types::prelude::*;
use futures_lite::StreamExt;
use kameo::{actor::ActorRef, error::Infallible, prelude::*};
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct SessionServiceActor {
    pub session_repository: SessionRepository,
    pub player_repository: PlayerRepository,
    pub messenger: Messenger,
}

impl SessionServiceActor {
    pub fn new(
        session_repository: &SessionRepository,
        player_repository: &PlayerRepository,
        messenger: &Messenger,
    ) -> Self {
        Self {
            session_repository: session_repository.clone(),
            player_repository: player_repository.clone(),
            messenger: messenger.clone(),
        }
    }

    pub async fn start_consuming(&self) -> Result<()> {
        info!("Starting JetStream message consumption for SessionService");

        let stream = self.messenger.js.get_stream("USER_EVENTS").await?;

        let consumer = stream
            .get_or_create_consumer(
                "session_service_consumer",
                async_nats::jetstream::consumer::pull::Config {
                    durable_name: Some("session_service_consumer".to_string()),
                    filter_subject: "user.logged.*".to_string(), // Matches both logged.in and logged.out
                    deliver_policy: async_nats::jetstream::consumer::DeliverPolicy::All,
                    ack_policy: async_nats::jetstream::consumer::AckPolicy::Explicit,
                    max_deliver: 3,
                    ..Default::default()
                },
            )
            .await?;

        let mut messages = consumer.messages().await?;
        while let Some(message) = messages.next().await {
            match message {
                Ok(msg) => {
                    // Handle the message first, then acknowledge
                    match self.handle_nats_message(msg.message.clone()).await {
                        Ok(_) => {
                            // Acknowledge only after successful processing
                            if let Err(e) = msg.ack().await {
                                error!("Failed to acknowledge message: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to handle JetStream message: {}", e);
                            // Optionally, you can choose to ack or nack the message
                            // For now, let's nack it so it can be redelivered
                            if let Err(ack_e) = msg.double_ack().await {
                                error!("Failed to nack message: {}", ack_e);
                            }
                        }
                    }
                }
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
            UserEvent::Login(token_user_data) => {
                info!(
                    "SessionService Received Login event for user: {}",
                    token_user_data.user.username
                );
                self.handle_user_login(token_user_data).await?;
            }
            UserEvent::Logout(token_data) => {
                info!(
                    "SessionService Received Logout event for token: {}",
                    token_data.token
                );
                self.handle_user_logout(token_data).await?;
            }
            UserEvent::TokenExpired(token_data) => {
                info!(
                    "SessionService Received TokenExpired event for token: {}",
                    token_data.token
                );
                self.handle_token_expired(token_data).await?;
            }
            _ => {
                // Ignore other user events (like Register)
            }
        }

        Ok(())
    }

    async fn handle_user_login(&self, token_user_data: TokenUserData) -> Result<()> {
        let username = &token_user_data.user.username;

        // Get player by username
        let player = match self
            .player_repository
            .get_player_by_username(username)
            .await?
        {
            Some(player) => player,
            None => {
                error!("No player found for user: {}", username);
                return Err(anyhow::anyhow!("No player found for user: {}", username));
            }
        };

        // End any existing active sessions for this player
        if let Some(existing_session) = self
            .session_repository
            .get_active_session_by_player_id(player.id)
            .await?
        {
            info!("Ending existing session for player: {}", username);
            self.session_repository
                .end_session(existing_session.id)
                .await?;
        }

        // Create new session
        let session = PlayerSession::new_with_token(
            player.id,
            username.clone(),
            token_user_data.token.token.clone(),
        );

        match self.session_repository.create_session(session).await {
            Ok(created_session) => {
                info!(
                    "Created session for player: {} (session_id: {})",
                    username, created_session.id
                );
                // TODO: produce sessions on NATS
            }
            Err(e) => {
                error!("Failed to create session for player {}: {}", username, e);
                return Err(e);
            }
        }

        Ok(())
    }

    async fn handle_user_logout(&self, token_data: TokenData) -> Result<()> {
        // Find session by token and end it
        if let Some(session) = self
            .session_repository
            .get_active_session_by_token(&token_data.token)
            .await?
        {
            self.session_repository.end_session(session.id).await?;
            info!(
                "Ended session for token: {} (session_id: {})",
                token_data.token, session.id
            );
        } else {
            warn!("No active session found for token: {}", token_data.token);
        }

        Ok(())
    }

    async fn handle_token_expired(&self, token_data: TokenData) -> Result<()> {
        // Find session by token and end it
        if let Some(session) = self
            .session_repository
            .get_active_session_by_token(&token_data.token)
            .await?
        {
            self.session_repository.end_session(session.id).await?;
            info!(
                "Ended session due to token expiration: {} (session_id: {})",
                token_data.token, session.id
            );
        } else {
            warn!(
                "No active session found for expired token: {}",
                token_data.token
            );
        }

        Ok(())
    }
}

impl Actor for SessionServiceActor {
    type Args = Self;
    type Error = Infallible;

    async fn on_start(state: Self::Args, _actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        // Start NATS consumption in background task
        let session_service_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = session_service_clone.start_consuming().await {
                error!("SessionService NATS consumption failed: {}", e);
            }
        });

        Ok(state)
    }
}

impl std::fmt::Debug for SessionServiceActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionServiceActor").finish()
    }
}
