use aeronet::io::{
    connection::{DisconnectReason, Disconnected},
    Session,
};
use bevy::prelude::*;
use bevy_replicon::prelude::{ConnectedClient, Replicated};

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_connected)
            .add_observer(on_disconnected)
            .add_observer(on_connected_client)
            .add_observer(on_disconnected_client);
    }
}

fn on_connected_client(
    trigger: Trigger<OnAdd, ConnectedClient>,
    q_conn_clients: Query<&ConnectedClient>,
) {
    if let Ok(client) = q_conn_clients.get(trigger.entity()) {
        info!(
            "Connected Entity: {:?} id: {:?}",
            trigger.entity(),
            client.id()
        );
    }
}

fn on_disconnected_client(trigger: Trigger<OnRemove, ConnectedClient>) {
    info!("Disconnected Entity: {:?}", trigger.entity());
}

fn on_connected(trigger: Trigger<OnAdd, Session>, clients: Query<&Parent>, mut commands: Commands) {
    let client = trigger.entity();
    let Ok(server) = clients.get(client).map(Parent::get) else {
        return;
    };

    info!("{client} Connected to {server}");
    commands.entity(client).insert(Replicated);
}

fn on_disconnected(trigger: Trigger<Disconnected>, clients: Query<&Parent>) {
    let client = trigger.entity();
    let Ok(server) = clients.get(client).map(Parent::get) else {
        return;
    };

    match &trigger.reason {
        DisconnectReason::User(reason) => {
            info!("{client} disconnected from {server} by user: {reason}");
        }
        DisconnectReason::Peer(reason) => {
            info!("{client} disconnected from {server} by peer: {reason}");
        }
        DisconnectReason::Error(err) => {
            warn!("{client} disconnected from {server} due to error: {err:?}");
        }
    }
}
