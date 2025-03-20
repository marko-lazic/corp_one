use crate::ServerState;
use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use corp_shared::prelude::*;
use lightyear::prelude::{server::*, *};
use rand::Rng;
use std::time::Duration;

pub struct ServerNetPlugin;

impl Plugin for ServerNetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientEntityMap>()
            .add_plugins((build_server_plugin(), ProtocolPlugin))
            .add_systems(Startup, start_server)
            .add_systems(
                FixedUpdate,
                backpack_spawner.run_if(on_timer(Duration::from_secs_f32(2.0))),
            )
            .add_systems(Update, (handle_connections, handle_disconnections));
    }
}

/// A simple resource map that tell me  the corresponding server entity of that client
/// Important for O(n) access
#[derive(Resource, Default)]
pub struct ClientEntityMap(HashMap<ClientId, Entity>);

fn backpack_spawner(
    mut commands: Commands,
    q_backpacks: Query<&Backpack>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    const MAX_BACKPACKS: usize = 10;
    if q_backpacks.iter().count() > MAX_BACKPACKS {
        return;
    }

    let x: f32 = rng.gen_range(-10.0..=10.0);
    let z: f32 = rng.gen_range(-10.0..=10.0);

    let e_item = commands.spawn((HackingTool, Replicate::default())).id();
    commands.spawn((
        Backpack,
        Inventory::new(vec![e_item]),
        Transform::from_xyz(x, 0.1, z),
        Replicate::default(),
    ));
}

pub(crate) fn handle_disconnections(
    mut commands: Commands,
    mut disconnections: EventReader<DisconnectEvent>,
    manager: Res<ConnectionManager>,
    client_query: Query<&ControlledEntities>,
) {
    for disconnection in disconnections.read() {
        debug!("Client {:?} disconnected", disconnection.client_id);
        if let Ok(client_entity) = manager.client_entity(disconnection.client_id) {
            if let Ok(controlled_entities) = client_query.get(client_entity) {
                for entity in controlled_entities.entities() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

/// Server connection system, create a player upon connection
fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut entity_map: ResMut<ClientEntityMap>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        // in host-server mode, server and client are running in the same app, no need to replicate to the local client
        let replicate = Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::Single(client_id),
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            ..default()
        };
        let entity = commands.spawn((PlayerId(client_id), CharacterMarker, replicate));
        entity_map.0.insert(client_id, entity.id());
        info!("Create entity {:?} for client {:?}", entity.id(), client_id);
    }
}

fn start_server(mut commands: Commands) {
    commands.start_server();
    commands.insert_resource(NextState::Pending(ServerState::Serve));
}

/// Here we create the lightyear [`ServerPlugins`]
fn build_server_plugin() -> ServerPlugins {
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the server_address, because we open a UDP socket on the server
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };
    let config = ServerConfig {
        shared: shared_config(),
        net: vec![net_config],
        ..default()
    };
    ServerPlugins::new(config)
}
