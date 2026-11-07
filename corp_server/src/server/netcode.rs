use crate::server::*;
use aeronet::io::{
    Session,
    connection::{DisconnectReason, Disconnected, LocalAddr},
    server::{Closed, Server},
};
use aeronet_replicon::server::{AeronetRepliconServer, AeronetRepliconServerPlugin};
use aeronet_webtransport::{
    cert,
    server::{SessionRequest, SessionResponse, WebTransportServer, WebTransportServerPlugin},
    wtransport,
};
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;
use std::time::Duration;

pub struct NetcodePlugin;

impl Plugin for NetcodePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // transport
            WebTransportServerPlugin,
            // replication
            RepliconPlugins.set(ServerPlugin {
                // 1 frame lasts `1.0 / TICK_RATE` anyway
                tick_schedule: Some(PostUpdate.intern()),
                ..Default::default()
            }),
            AeronetRepliconServerPlugin,
            ReplicateRulesPlugin,
        ))
        .add_systems(Startup, open_server)
        .add_observer(on_opened)
        .add_observer(on_closed)
        .add_observer(on_connected)
        .add_observer(on_disconnected);
    }
}

fn open_server(mut commands: Commands, game_server_config: Res<GameServerConfig>) {
    let identity = game_server_config.identity.clone_identity();
    let cert = identity
        .certificate_chain()
        .as_slice()
        .first()
        .expect("server identity should contain a certificate");
    let spki_fingerprint = cert::spki_fingerprint_b64(cert).expect("should be a valid certificate");
    let cert_hash = cert::hash_to_b64(cert.hash());
    debug!(colony = ?game_server_config.colony, %spki_fingerprint, %cert_hash, "WebTransport certificate");

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
    debug!("Opening WebTransport server \"{server}\"");
}

fn on_opened(
    trigger: On<Add<Server>>,
    servers: Query<&LocalAddr>,
    config: Res<GameServerConfig>,
    mut commands: Commands,
) {
    let server = trigger.event_target();
    let local_addr = servers
        .get(server)
        .expect("opened server should have a LocalAddr");

    info!(
        "{:?} server \"{server}\" opened on {}",
        config.colony, **local_addr
    );
    commands.entity(server).observe(on_session_request);
}

fn on_closed(closed: On<Closed>) {
    panic!("server closed: {:?}", closed.event());
}

fn on_session_request(
    mut request: On<SessionRequest>,
    tokens: Res<Tokens>,
    mut commands: Commands,
) {
    let server = request.event_target();
    let client = request.session_entity;

    debug!(
        "{client} connecting to {server} from {} with headers:",
        request.remote_addr
    );

    for (header_key, header_value) in &request.headers {
        if header_key == "x-token" {
            debug!("  {header_key}: [redacted]");
        } else {
            debug!("  {header_key}: {header_value}");
        }
    }

    let token = request.headers.get("x-token").cloned().unwrap_or_default();

    if tokens.is_valid(&token) {
        debug!("Accepted connection from {client}");
        commands.entity(client).insert(SessionAuthToken(token));
        request.respond(SessionResponse::Accepted);
    } else {
        warn!("Rejected connection from {client}: invalid token");
        request.respond(SessionResponse::NotFound);
    }
}

fn on_connected(
    trigger: On<Add<Session>>,
    clients: Query<&ChildOf>,
    config: Res<GameServerConfig>,
) -> Result {
    let client = trigger.event_target();
    let &ChildOf(server) = clients.get(client)?;
    let server_colony = config.colony;

    info!("Session \"{client}\" connected to server {server_colony} \"{server}\"");
    Ok(())
}

fn on_disconnected(
    disconnected: On<Disconnected>,
    clients: Query<&ChildOf>,
    config: Res<GameServerConfig>,
) {
    let client = disconnected.entity;
    let Ok(&ChildOf(server)) = clients.get(client) else {
        error!("\"{client}\" disconnected from server but it's not a child of it");
        return;
    };
    let server_colony = config.colony;

    match &disconnected.reason {
        DisconnectReason::ByUser(reason) => {
            info!("\"{client}\" disconnected from {server_colony} \"{server}\" by user: {reason}");
        }
        DisconnectReason::ByPeer(reason) => {
            info!("\"{client}\" disconnected from {server_colony} \"{server}\" by peer: {reason}");
        }
        DisconnectReason::ByError(err) => {
            warn!(
                "\"{client}\" disconnected from {server_colony} \"{server}\" due to error: {err:?}"
            );
        }
    }
}
