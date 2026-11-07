use crate::{channel::ChannelSender, integration::RETRY_DELAY, server::BevyUserEvent};
use async_nats::jetstream::{Message, consumer::pull};
use bevy::tasks::futures_lite::StreamExt;
use corp_types::{auth::UserEvent, messenger::Messenger};
use tracing::{debug, error, info};

pub struct TokenService {
    sender: ChannelSender<BevyUserEvent>,
    messenger: Messenger,
    channel_name: String,
}

impl TokenService {
    pub fn new(
        sender: ChannelSender<BevyUserEvent>,
        messenger: Messenger,
        channel_name: String,
    ) -> Self {
        Self {
            sender,
            messenger,
            channel_name,
        }
    }

    pub async fn run(&mut self) {
        if let Err(e) = self.consume_user_events().await {
            error!("Error consuming user events: {}", e);
        }
    }

    async fn consume_user_events(&self) -> anyhow::Result<()> {
        loop {
            match self.messenger.js.get_stream("USER_EVENTS").await {
                Ok(stream) => {
                    debug!("Successfully connected to USER_EVENTS stream");

                    debug!("Subscribing to consumer {}", self.channel_name);
                    if let Ok(consumer) = stream
                        .get_consumer::<pull::Config>(&self.channel_name)
                        .await
                    {
                        if let Ok(mut messages) = consumer.messages().await {
                            while let Some(result) = messages.next().await {
                                match result {
                                    Ok(msg) => match self.handle_user_event_message(&msg).await {
                                        Ok(_) => {
                                            if let Err(e) = msg.ack().await {
                                                error!("Failed to acknowledge message: {}", e);
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to handle UserEvent message: {}", e);
                                            if let Err(ack_e) = msg.double_ack().await {
                                                error!("Failed to nack message: {}", ack_e);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("Error receiving NATS message: {}", e);
                                    }
                                }
                            }
                        } else {
                            info!("Failed to get messages {}", self.channel_name);
                        }
                    } else {
                        info!("Failed to get consumer {}", self.channel_name);
                    }
                }
                Err(e) => {
                    error!("Failed to get stream USER_EVENTS: {}", e);
                    info!("Retrying in {}ms...", RETRY_DELAY.as_millis());
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    }

    async fn handle_user_event_message(&self, message: &Message) -> anyhow::Result<()> {
        let user_event: UserEvent = rmp_serde::from_slice(&message.payload)?;
        debug!(consumer = %self.channel_name, "Received user authentication event");
        self.sender.send(BevyUserEvent(user_event));
        Ok(())
    }
}
