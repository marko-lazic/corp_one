use crate::server::*;
use aeronet::io::{
    connection::{Disconnected, LocalAddr},
    server::{Closed, Server},
    Session,
};
use aeronet_replicon::server::{AeronetRepliconServer, AeronetRepliconServerPlugin};
use aeronet_webtransport::{
    cert,
    server::{SessionRequest, SessionResponse, WebTransportServer, WebTransportServerPlugin},
    wtransport,
};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;
use corp_types::prelude::*;
use std::time::Duration;
use surf::http::convert::json;

pub struct ServerNetPlugin;

impl Plugin for ServerNetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // transport
            WebTransportServerPlugin,
            // replication
            RepliconPlugins.set(ServerPlugin {
                // 1 frame lasts `1.0 / TICK_RATE` anyway
                tick_policy: TickPolicy::EveryFrame,
                ..Default::default()
            }),
            AeronetRepliconServerPlugin,
            ReplicateRulesPlugin,
        ))
        .add_systems(Startup, open_server)
        .add_observer(on_opened)
        .add_observer(on_closed)
        .add_observer(on_session_request)
        .add_observer(on_connected)
        .add_observer(on_disconnected);
    }
}

fn open_server(mut commands: Commands, game_server_config: Res<GameServerConfig>) {
    let identity = game_server_config.identity.clone_identity();
    let cert = &identity.certificate_chain().as_slice()[0];
    let spki_fingerprint = cert::spki_fingerprint_b64(cert).expect("should be a valid certificate");
    let cert_hash = cert::hash_to_b64(cert.hash());
    info!("************************");
    info!("SPKI FINGERPRINT");
    info!("  {spki_fingerprint}");
    info!("CERTIFICATE HASH");
    info!("  {cert_hash}");
    info!("************************");

    let config = wtransport::ServerConfig::builder()
        .with_bind_default(game_server_config.server_addr.port())
        .with_identity(identity)
        .keep_alive_interval(Some(Duration::from_secs(1)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build();
    let server = commands
        .spawn((Name::new("WebTransport Server"), AeronetRepliconServer))
        .queue(WebTransportServer::open(config))
        .id();
    info!("Opening WebTransport server \"{server}\"");
}

fn on_opened(trigger: Trigger<OnAdd, Server>, servers: Query<&LocalAddr>) {
    let server = trigger.target();
    let local_addr = servers
        .get(server)
        .expect("spawned session entity should have a name");
    info!("\"{server}\" opened on {}", **local_addr);
}

fn on_closed(trigger: Trigger<Closed>) {
    panic!("server closed: {:?}", trigger.event());
}

fn on_session_request(
    mut request: Trigger<SessionRequest>,
    clients: Query<&ChildOf>,
    game_server_config: Res<GameServerConfig>,
    mut commands: Commands,
) -> Result {
    let client = request.target();
    let &ChildOf(server) = clients.get(client)?;

    debug!("\"{client}\" connecting to \"{server}\" with headers:");

    let mut token = String::new();
    for (header_key, header_value) in &request.headers {
        debug!("  {header_key}: {header_value}");

        // Extract username and password from headers
        match header_key.as_str() {
            "x-token" => token = header_value.clone(),
            _ => {}
        }
    }
    match game_server_config
        .tokens_ref
        .ask(IsTokenValid(token.clone()))
        .blocking_send()
    {
        Ok(true) => {
            info!("Accepted token {} from {:?}", token, client);
            commands.entity(client).insert(AuthToken(token));
            request.respond(SessionResponse::Accepted);
        }
        Ok(false) => {
            warn!("Rejected token \"{}\" from {:?}", token, client);
            request.respond(SessionResponse::NotFound)
        }
        Err(e) => {
            warn!(
                "Error while validating token {} from {:?} reason: {:?}",
                token, client, e
            );
            request.respond(SessionResponse::NotFound)
        }
    }
    Ok(())
}

fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    clients: Query<&ChildOf>,
    config: Res<GameServerConfig>,
) -> Result {
    let client = trigger.target();
    let &ChildOf(server) = clients.get(client)?;
    let server_colony = config.colony;
    info!("Session \"{client}\" connected to server {server_colony} \"{server}\"");
    // Add client metadata
    Ok(())
}

fn on_disconnected(
    trigger: Trigger<Disconnected>,
    clients: Query<&ChildOf>,
    config: Res<GameServerConfig>,
) {
    let client = trigger.target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        error!("\"{client}\" disconnected from server but it's not a child of it");
        return;
    };
    let server_colony = config.colony;

    match &*trigger {
        Disconnected::ByUser(reason) => {
            info!("\"{client}\" disconnected from {server_colony} \"{server}\" by user: {reason}");
        }
        Disconnected::ByPeer(reason) => {
            info!("\"{client}\" disconnected from {server_colony} \"{server}\" by peer: {reason}");
        }
        Disconnected::ByError(err) => {
            warn!(
                "\"{client}\" disconnected from {server_colony} \"{server}\" due to error: {err:?}"
            );
        }
    }
}
