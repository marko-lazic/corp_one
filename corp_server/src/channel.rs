use bevy::prelude::*;
use tokio::sync::mpsc::{self, error::TryRecvError, UnboundedReceiver, UnboundedSender};

/// channel sender to share with multiple producers and offering a simple `send` function
#[derive(Resource, Clone, Debug)]
pub struct ChannelSender<T: Event>(UnboundedSender<T>);

impl<T: Event> ChannelSender<T> {
    /// send `event` to our central receiver that forwards them as triggers that can be observed
    pub fn send(&self, event: impl Into<T>) {
        let event = event.into();
        if let Err(err) = self.0.send(event) {
            error!("sending failed due to receiver being dropped: {err:?}");
        };
    }
}

#[derive(Resource)]
struct EventReceiver<T: Event>(UnboundedReceiver<T>);

/// Extension to Bevy `App` that allows ergonomic creation of the channel
pub trait ChannelTriggerApp {
    /// Spawns a channel registers the receiver as a resource and returns the `ChannelSender<T>`
    /// This sender can be used from anywhere to send events into the Bevy world.
    /// These triggers can be subscribed to via `app.observe`.
    fn add_channel_trigger<'a, T: Event>(&mut self) -> ChannelSender<T>;
}

impl ChannelTriggerApp for App {
    fn add_channel_trigger<'a, T: Event>(&mut self) -> ChannelSender<T> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.insert_resource(EventReceiver::<T>(receiver));
        self.add_systems(PreUpdate, process_events::<T>);
        ChannelSender::<T>(sender)
    }
}

fn process_events<'a, T: Event>(
    receiver: Option<ResMut<EventReceiver<T>>>,
    mut commands: Commands,
) {
    if let Some(mut receiver) = receiver {
        loop {
            match receiver.0.try_recv() {
                Ok(msg) => {
                    commands.trigger(msg);
                }
                Err(TryRecvError::Disconnected) => {
                    error!("sender dropped, removing receiver");
                    commands.remove_resource::<EventReceiver<T>>();
                    break;
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
            }
        }
    }
}
