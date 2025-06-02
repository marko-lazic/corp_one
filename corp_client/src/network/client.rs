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

#[derive(Event)]
pub struct RequestConnect(pub Colony);
#[derive(Component)]
pub struct Client;

#[derive(Component)]
pub struct ConnectedPlayer;

pub struct ClientNetPlugin;

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
        .add_systems(OnExit(GameState::Playing), disconnect_client)
        .add_systems(OnExit(GameState::StarMap), disconnect_client)
        .add_systems(FixedUpdate, graceful_disconnect_on_exit)
        .add_observer(request_connect)
        .add_observer(connect_client)
        .add_observer(on_connecting)
        .add_observer(on_connected)
        .add_observer(on_disconnected);
    }
}

fn request_connect(
    trigger: Trigger<RequestConnect>,
    mut commands: Commands,
    mut r_next_game_state: ResMut<NextState<GameState>>,
    q_client_entity: Query<Entity, With<Client>>,
) -> Result {
    if let Ok(e_client) = q_client_entity.single() {
        commands.entity(e_client).try_despawn();
    }
    commands.trigger(ConnectClient(trigger.0));
    r_next_game_state.set(GameState::Loading);
    Ok(())
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

fn disconnect_client(
    mut commands: Commands,
    session_endpoint: Single<(Entity, &Name, Option<&Session>), With<SessionEndpoint>>,
    connected_player_entity: Single<Entity, With<ConnectedPlayer>>,
    client_entity: Single<Entity, With<Client>>,
) -> Result {
    let (session, name, session_opt) = *session_endpoint;

    if session_opt.is_some() {
        info!("{name} is Connected");
        commands.trigger_targets(
            Disconnect::new("Disconnected by User - Changing State"),
            session,
        );
        commands.entity(*connected_player_entity).try_despawn();
        commands.entity(*client_entity).try_despawn();
    } else {
        info!("{name} is not Connected");
    }

    Ok(())
}

fn graceful_disconnect_on_exit(
    mut exit_ev: EventReader<AppExit>,
    q_sessions: Query<(Entity, Option<&Session>), With<SessionEndpoint>>,
    mut commands: Commands,
) {
    // if the AppExit event was sent…
    if exit_ev.read().next().is_some() {
        for (entity, session_opt) in q_sessions.iter() {
            if session_opt.is_some() {
                info!("Disconnected by User - App is shutting down");
                commands.trigger_targets(Disconnect::new("App exiting"), entity);
            }
        }
    }
}

fn on_disconnected(
    trigger: Trigger<Disconnected>,
    names: Query<&Name>,
    mut game_state: ResMut<NextState<GameState>>,
) -> Result {
    let target = trigger.target();
    let name = names.get(target)?;
    match trigger.event() {
        Disconnected::ByUser(reason) => {
            info!("{name} disconnected by user: {reason}")
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
