use crate::prelude::*;
use aeronet::{
    io::{
        connection::{Disconnect, Disconnected}, Session,
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
#[derive(Event, Deref)]
pub struct RequestConnect(pub Colony);
#[derive(Event)]
pub struct RequestExit;
pub struct ClientNetPlugin;
#[derive(Event)]
struct RequestDisconnect;
#[derive(Event)]
struct ConnectClientTo(pub Colony);
#[derive(Event, Clone, Eq, PartialEq)]
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
        .add_event::<ConnectionDisconnectedEvent>()
        .react_to_event::<ConnectionDisconnectedEvent>()
        .init_resource::<ClientSettings>()
        .add_systems(Startup, setup_client)
        .add_observer(connect_client)
        .add_observer(on_connecting)
        .add_observer(on_connected)
        .add_observer(request_disconnect)
        .add_observer(disconnect_and_exit)
        .add_observer(on_disconnected);
    }
}

fn setup_client(mut commands: Commands) {
    commands.spawn(CorpClient).observe(request_connect);
}

fn request_connect(trigger: Trigger<RequestConnect>, mut commands: Commands) {
    let colony = **trigger;
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
    trigger: Trigger<ConnectClientTo>,
    mut commands: Commands,
    client_settings: Res<ClientSettings>,
    token: Option<Res<AuthToken>>,
) -> Result {
    let colony = trigger.event().0;
    let config = client_settings.client_config();

    let token = token.ok_or("Auth token not found")?;
    let connect_options = client_settings.target(&colony, &token);
    info!("client_connect with options: {connect_options:?}");
    commands
        .spawn((
            Name::new(format!("Client Session {}", colony)),
            colony,
            AeronetRepliconClient,
        ))
        .queue(WebTransportClient::connect(config, connect_options));
    Ok(())
}

fn on_connecting(trigger: Trigger<OnAdd, SessionEndpoint>, names: Query<&Name>) -> Result {
    let target = trigger.target();
    let name = names.get(target)?;
    info!("SessionEndpoint \"{name}\" connecting...");
    Ok(())
}

fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    mut commands: Commands,
    client_colony: Single<&Colony, With<AeronetRepliconClient>>,
) -> Result {
    let e_session = trigger.target();
    info!("Session {e_session} Connected!");

    commands.entity(e_session).insert((TransportConfig {
        max_memory_usage: 64 * 1024,
        send_bytes_per_sec: 4 * 1024,
        ..default()
    },));

    if **client_colony == Colony::StarMap {
        commands.trigger(LoadStarMapCommand);
    } else {
        commands.trigger(LoadColonyCommand);
    }
    Ok(())
}

fn request_disconnect(
    _trigger: Trigger<RequestDisconnect>,
    mut commands: Commands,
    session_endpoint: Query<(Entity, &Name, Option<&Session>), With<SessionEndpoint>>,
) {
    if let Ok((session, name, session_opt)) = session_endpoint.single() {
        // Handle disconnect logic
        if session_opt.is_some() {
            info!("Session \"{name}\" is Connected, sending disconnect request");
            commands.trigger_targets(Disconnect::new(code::REQUEST_DISCONNECT), session);
        } else {
            info!("Session \"{name}\" is not Connected!");
            commands.send_event(ConnectionDisconnectedEvent);
        }
    } else {
        info!("No session endpoint found, sending disconnected event");
        commands.send_event(ConnectionDisconnectedEvent);
    }
}

fn disconnect_and_exit(
    _trigger: Trigger<RequestExit>,
    s_session_entity: Single<(Entity, Option<&Session>), With<SessionEndpoint>>,
    mut commands: Commands,
    mut exit_ev: EventWriter<AppExit>,
) {
    let (entity, session_opt) = *s_session_entity;
    if session_opt.is_some() {
        commands.trigger_targets(Disconnect::new(code::APP_EXIT), entity);
    } else {
        exit_ev.write(AppExit::Success);
    }
}

fn on_disconnected(
    trigger: Trigger<Disconnected>,
    names: Query<&Name>,
    mut commands: Commands,
    mut exit_ev: EventWriter<AppExit>,
) -> Result {
    let target = trigger.target();
    let name = names.get(target)?;
    match trigger.event() {
        Disconnected::ByUser(reason) => {
            info!("{name} disconnected by user: {reason}");
            if reason == code::APP_EXIT {
                exit_ev.write(AppExit::Success);
            } else if reason == code::REQUEST_DISCONNECT {
                commands.send_event(ConnectionDisconnectedEvent);
            }
        }
        Disconnected::ByPeer(reason) => {
            info!("{name} disconnected by peer: {reason}")
        }
        Disconnected::ByError(err) => {
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
