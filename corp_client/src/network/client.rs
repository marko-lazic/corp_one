use crate::prelude::*;
use aeronet::{
    io::{
        connection::{DisconnectReason, Disconnected},
        Session, SessionEndpoint,
    },
    transport::TransportConfig,
};
use aeronet_replicon::client::{AeronetRepliconClient, AeronetRepliconClientPlugin};
use aeronet_webtransport::{
    cert,
    client::{WebTransportClient, WebTransportClientPlugin},
    wtransport,
    wtransport::tls::Sha256Digest,
};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use core::time::Duration;
use corp_shared::prelude::*;

pub struct ClientNetPlugin;

impl Plugin for ClientNetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // transport
            WebTransportClientPlugin,
            // replication
            RepliconPlugins,
            AeronetRepliconClientPlugin,
            ReplicateRulesPlugin,
            // game
            SpawnListenerPlugin,
        ))
        .add_systems(OnEnter(LoadingSubState::Loaded), connect_client)
        .add_observer(on_connecting)
        .add_observer(on_connected)
        .add_observer(on_disconnected);
    }
}

fn connect_client(mut commands: Commands) {
    let cert_hash = "".to_string();
    let config = web_transport_config(cert_hash);

    let target = "https://[::1]:25560".to_string();

    let name = "Corp One Session ID 1";
    commands
        .spawn((Name::new(name), AeronetRepliconClient))
        .queue(WebTransportClient::connect(config, target));
}

fn web_transport_config(cert_hash: String) -> wtransport::ClientConfig {
    let config = wtransport::ClientConfig::builder().with_bind_default();

    let config = if cert_hash.is_empty() {
        warn!("Connecting without certificate validation");
        config.with_no_cert_validation()
    } else {
        match cert::hash_from_b64(&cert_hash) {
            Ok(hash) => config.with_server_certificate_hashes([Sha256Digest::new(hash)]),
            Err(err) => {
                warn!("Failed to read certificate hash from string: {err:?}");
                config.with_server_certificate_hashes([])
            }
        }
    };

    config
        .keep_alive_interval(Some(Duration::from_secs(1)))
        .max_idle_timeout(Some(Duration::from_secs(20)))
        .expect("should be a valid idle timeout")
        .build()
}

fn on_connecting(trigger: Trigger<OnAdd, SessionEndpoint>, names: Query<&Name>) {
    let entity = trigger.entity();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");
    info!("{name} connecting");
}

fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    names: Query<&Name>,
    _game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    r_player_systems: Res<PlayerSystems>,
) {
    let entity = trigger.entity();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");
    info!("{name} connected");

    commands.entity(entity).insert((TransportConfig {
        max_memory_usage: 64 * 1024,
        send_bytes_per_sec: 4 * 1024,
        ..default()
    },));

    commands.run_system_with_input(r_player_systems.setup_player, trigger.entity());
}

fn on_disconnected(
    trigger: Trigger<Disconnected>,
    names: Query<&Name>,
    _game_state: ResMut<NextState<GameState>>,
) {
    let session = trigger.entity();
    let name = names
        .get(session)
        .expect("our session entity should have a name");
    match &trigger.reason {
        DisconnectReason::User(reason) => {
            info!("{name} disconnected by user: {reason}")
        }
        DisconnectReason::Peer(reason) => {
            info!("{name} disconnected by peer: {reason}")
        }
        DisconnectReason::Error(err) => {
            info!("{name} disconnected due to error: {err:?}")
        }
    };
}
