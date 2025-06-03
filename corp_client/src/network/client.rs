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
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;

#[derive(Event, Deref)]
pub struct RequestConnect(pub Colony);
#[derive(Event)]
pub struct RequestExit;
#[derive(Component)]
pub struct Client;
#[derive(Component)]
pub struct ConnectedPlayer;
pub struct ClientNetPlugin;
#[derive(Event)]
struct RequestDisconnect;
#[derive(Event)]
struct ConnectClient(pub Colony);

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
        .init_resource::<ClientSettings>()
        .add_observer(request_connect)
        .add_observer(connect_client)
        .add_observer(on_connecting)
        .add_observer(on_connected)
        .add_observer(request_disconnect)
        .add_observer(disconnect_and_exit)
        .add_observer(on_disconnected);
    }
}

fn request_connect(
    colony: Trigger<RequestConnect>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game_state.set(GameState::Loading);
    commands.trigger(RequestDisconnect);
    commands.trigger(ConnectClient(**colony));
}

fn connect_client(
    trigger: Trigger<ConnectClient>,
    mut commands: Commands,
    client_settings: Res<ClientSettings>,
) -> Result {
    let colony = trigger.0;
    let config = client_settings.client_config();
    let target = client_settings.target(colony);
    info!("Connecting with {target:?}");
    commands
        .spawn((
            Client,
            Name::new(format!("Client Session {}", colony)),
            colony,
            AeronetRepliconClient,
        ))
        .queue(WebTransportClient::connect(config, target));
    Ok(())
}

fn on_connecting(trigger: Trigger<OnAdd, SessionEndpoint>, names: Query<&Name>) -> Result {
    let target = trigger.target();
    let name = names.get(target)?;
    info!("{name} Connecting");
    Ok(())
}

fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    mut r_next_loading_state: ResMut<NextState<LoadingState>>,
    mut commands: Commands,
) -> Result {
    let e_session = trigger.target();
    info!("Session {e_session} Connected!");

    commands.entity(e_session).insert((
        ConnectedPlayer,
        TransportConfig {
            max_memory_usage: 64 * 1024,
            send_bytes_per_sec: 4 * 1024,
            ..default()
        },
    ));
    r_next_loading_state.set(LoadingState::LoadColony);
    Ok(())
}

fn request_disconnect(
    _trigger: Trigger<RequestDisconnect>,
    mut commands: Commands,
    session_endpoint: Single<(Entity, &Name, Option<&Session>), With<SessionEndpoint>>,
    client_entity: Query<Entity, With<Client>>,
) -> Result {
    let (session, name, session_opt) = *session_endpoint;

    if session_opt.is_some() {
        info!("{name} is Connected");
        commands.trigger_targets(Disconnect::new(code::REQUEST_DISCONNECT), session);
    } else {
        info!("{name} is not Connected");
    }
    if let Ok(existing) = client_entity.single() {
        commands.entity(existing).try_despawn();
    }

    Ok(())
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
    mut game_state: ResMut<NextState<GameState>>,
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
            }
        }
        Disconnected::ByPeer(reason) => {
            info!("{name} disconnected by peer: {reason}")
        }
        Disconnected::ByError(err) => {
            info!("{name} disconnected due to error: {err:?}");
            game_state.set(GameState::Login);
        }
    };
    Ok(())
}

mod code {
    pub const APP_EXIT: &str = "APP_EXIT";
    pub const REQUEST_DISCONNECT: &str = "REQUEST_DISCONNECT";
}
