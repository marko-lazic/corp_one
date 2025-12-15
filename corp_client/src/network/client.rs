use crate::prelude::*;
use aeronet::{
    io::{
        connection::{Disconnect, DisconnectReason, Disconnected}, Session,
        SessionEndpoint,
    },
    transport::TransportConfig,
};
use aeronet_replicon::client::{AeronetRepliconClient, AeronetRepliconClientPlugin};
use aeronet_webtransport::client::{WebTransportClient, WebTransportClientPlugin};
use bevy::prelude::*;
use bevy_defer::{AppReactorExtension, AsyncCommandsExtension, AsyncWorld};
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;
#[derive(Component)]
pub struct CorpClient;
#[derive(Component, Debug, Clone, Copy)]
pub struct PendingColony(pub Colony);
#[derive(Event, Deref)]
pub struct RequestConnect(pub Colony);
#[derive(Event)]
pub struct RequestExit;
pub struct ClientNetPlugin;
#[derive(Event)]
struct RequestDisconnect;
#[derive(Event)]
struct ConnectClientTo(pub Colony);
#[derive(Message, Clone, Eq, PartialEq)]
struct ConnectionDisconnectedEvent;

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
        .add_message::<ConnectionDisconnectedEvent>()
        .react_to_message::<ConnectionDisconnectedEvent>()
        .init_resource::<ClientSettings>()
        .add_systems(Startup, setup_client)
        .add_observer(request_connect)
        .add_observer(connect_client)
        .add_observer(on_connecting)
        .add_observer(on_connected)
        .add_observer(request_disconnect)
        .add_observer(disconnect_and_exit)
        .add_observer(on_disconnected);
    }
}

fn setup_client(mut commands: Commands) {
    commands.spawn(CorpClient);
}

fn request_connect(request_connect: On<RequestConnect>, mut commands: Commands) {
    let colony = **request_connect;
    info!("request_connect to: {:?}", colony);
    commands.spawn_task(move || async move {
        // First trigger disconnect
        AsyncWorld.apply_command(|w: &mut World| {
            w.trigger(RequestDisconnect);
        });

        // Then wait for disconnection and handle reconnection
        loop {
            let _event = AsyncWorld.next_event::<ConnectionDisconnectedEvent>().await;
            info!("request_connect disconnected, reconnecting");
            AsyncWorld.set_state(GameState::Loading)?;
            AsyncWorld.apply_command(move |w: &mut World| {
                w.trigger(ConnectClientTo(colony));
            });
            break;
        }
        Ok(())
    });
}

fn connect_client(
    connect_client_to: On<ConnectClientTo>,
    mut commands: Commands,
    client_settings: Res<ClientSettings>,
    token: Option<Res<AuthToken>>,
) -> Result {
    let colony = connect_client_to.event().0;
    let config = client_settings.client_config();

    let token = token.ok_or("Auth token not found")?;
    let connect_options = client_settings.target(&colony, &token);
    info!("client_connect with options: {connect_options:?}");
    commands
        .spawn((
            Name::new(format!("Client Session {}", colony)),
            PendingColony(colony),
            AeronetRepliconClient,
        ))
        .queue(WebTransportClient::connect(config, connect_options));
    Ok(())
}

fn on_connecting(add: On<Add, SessionEndpoint>, names: Query<&Name>) -> Result {
    let name = names.get(add.entity)?;
    info!("SessionEndpoint \"{name}\" connecting...");
    Ok(())
}

fn on_connected(
    add: On<Add, Session>,
    mut commands: Commands,
    client_colony: Single<&PendingColony, With<AeronetRepliconClient>>,
) -> Result {
    let e_session = add.entity;
    let colony = client_colony.0;
    info!("Session {colony} Connected!");

    commands.entity(e_session).insert((TransportConfig {
        max_memory_usage: 64 * 1024,
        tx_bytes_per_sec: 4 * 1024,
        ..default()
    },));

    if colony == Colony::StarMap {
        commands.trigger(LoadStarMapCommand);
    } else {
        commands.trigger(LoadColonyCommand);
    }
    Ok(())
}

fn request_disconnect(
    _request_disconnect: On<RequestDisconnect>,
    mut commands: Commands,
    session_endpoint: Query<(Entity, &Name, Option<&Session>), With<SessionEndpoint>>,
) {
    if let Ok((session, name, session_opt)) = session_endpoint.single() {
        // Handle disconnect logic
        if session_opt.is_some() {
            info!("Session \"{name}\" is Connected, sending disconnect request");
            commands.trigger(Disconnect::new(session, code::REQUEST_DISCONNECT));
        } else {
            info!("Session \"{name}\" is not Connected!");
            commands.write_message(ConnectionDisconnectedEvent);
        }
    } else {
        info!("No session endpoint found, sending disconnected event");
        commands.write_message(ConnectionDisconnectedEvent);
    }
}

fn disconnect_and_exit(
    _request_exit: On<RequestExit>,
    s_session_entity: Single<(Entity, Option<&Session>), With<SessionEndpoint>>,
    mut commands: Commands,
    mut exit_ev: MessageWriter<AppExit>,
) {
    let (entity, session_opt) = *s_session_entity;
    if session_opt.is_some() {
        commands.trigger(Disconnect::new(entity, code::APP_EXIT));
    } else {
        exit_ev.write(AppExit::Success);
    }
}

fn on_disconnected(
    disconnected: On<Disconnected>,
    names: Query<&Name>,
    mut commands: Commands,
    mut exit_ev: MessageWriter<AppExit>,
) -> Result {
    let target = disconnected.entity;
    let name = names.get(target)?;
    match &disconnected.event().reason {
        DisconnectReason::ByUser(reason) => {
            info!("{name} disconnected by user: {reason}");
            if reason == code::APP_EXIT {
                exit_ev.write(AppExit::Success);
            } else if reason == code::REQUEST_DISCONNECT {
                commands.write_message(ConnectionDisconnectedEvent);
            }
        }
        DisconnectReason::ByPeer(reason) => {
            info!("{name} disconnected by peer: {reason}")
        }
        DisconnectReason::ByError(err) => {
            info!("{name} disconnected due to error: {err:?}");
            commands.set_state(GameState::Login);
        }
    };
    Ok(())
}

mod code {
    pub const APP_EXIT: &str = "APP_EXIT";
    pub const REQUEST_DISCONNECT: &str = "REQUEST_DISCONNECT";
}
