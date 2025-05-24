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
use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_replicon::prelude::*;
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
        .init_resource::<ClientSettings>()
        .add_systems(OnEnter(LoadingSubState::Connect), connect_client)
        .add_systems(OnExit(GameState::Playing), disconnect_client)
        .add_systems(OnExit(GameState::StarMap), disconnect_client)
        .add_systems(FixedUpdate, graceful_disconnect_on_exit)
        .add_observer(on_connecting)
        .add_observer(on_connected)
        .add_observer(on_disconnected);
    }
}

fn connect_client(
    mut commands: Commands,
    client_settings: Res<ClientSettings>,
    r_state: Res<State<GameState>>,
) {
    if let Some(colony) = r_state.get_loading_colony() {
        let config = client_settings.client_config();
        let target = client_settings.target(*colony);
        info!("Connecting with {target:?}");
        commands
            .spawn((
                Name::new(format!("Client Session {}", colony)),
                *colony,
                AeronetRepliconClient,
            ))
            .queue(WebTransportClient::connect(config, target));
    } else {
        error!("Client failed to connect in {:?} state", r_state.get());
    };
}

fn on_connecting(trigger: Trigger<OnAdd, SessionEndpoint>, names: Query<&Name>) -> Result {
    let target = trigger.target();
    let name = names.get(target)?;
    info!("{name} Connecting");
    Ok(())
}

fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    r_state: Res<State<GameState>>,
    mut r_next_state: ResMut<NextState<GameState>>,
    names: Query<&Name>,
    mut r_player_entity: ResMut<PlayerEntity>,
    mut r_next_loading_sub_state: ResMut<NextState<LoadingSubState>>,
    mut commands: Commands,
) -> Result {
    let target = trigger.target();
    let name = names.get(target)?;
    info!("{name} Connected");

    commands.entity(target).insert((TransportConfig {
        max_memory_usage: 64 * 1024,
        send_bytes_per_sec: 4 * 1024,
        ..default()
    },));

    *r_player_entity = PlayerEntity::from(trigger.target());

    if r_state
        .get_loading_colony()
        .map(|c| c.is_star_map())
        .unwrap_or_default()
    {
        r_next_state.set(GameState::StarMap);
    } else {
        r_next_loading_sub_state.set(LoadingSubState::SpawnPlayer);
    }
    Ok(())
}

fn disconnect_client(
    mut commands: Commands,
    mut r_player_entity: ResMut<PlayerEntity>,
    q_session: Single<(Entity, &Name, Option<&Session>), With<SessionEndpoint>>,
) -> Result {
    let (session, name, session_opt) = *q_session;

    if session_opt.is_some() {
        info!("{name} is Connected");
        commands.trigger_targets(
            Disconnect::new("Disconnected by User - Changing State"),
            session,
        );
        r_player_entity.0 = None;
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
