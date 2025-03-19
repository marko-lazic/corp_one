use crate::prelude::PlayerSystems;
use bevy::prelude::*;
use corp_shared::prelude::*;
pub use lightyear::prelude::client::*;
use lightyear::{prelude::*, shared::replication::components::Controlled};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

pub struct ClientNetPlugin;

impl Plugin for ClientNetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((build_client_plugin(), ProtocolPlugin))
            .add_systems(OnEnter(GameState::LoadColony), connect_client)
            .add_systems(Update, (on_connect, handle_new_character))
            .add_systems(
                Update,
                (receive_entity_spawn).run_if(in_state(GameState::Playing)),
            );
    }
}

fn on_connect(mut connect_event: EventReader<ConnectEvent>) {
    for event in connect_event.read() {
        let client_id = event.client_id();
        info!("Received ConnectEvent: {:?}", client_id);
    }
}

pub fn receive_entity_spawn(mut reader: EventReader<EntitySpawnEvent>) {
    for event in reader.read() {
        info!("Received entity spawn: {:?}", event.entity());
    }
}

fn handle_new_character(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    mut character_query: Query<
        (Entity, Has<Controlled>),
        (Added<Predicted>, With<CharacterMarker>),
    >,
    r_player_systems: Res<PlayerSystems>,
) {
    for (entity, is_controlled) in &mut character_query {
        if is_controlled {
            info!("Adding Player setup to controlled and predicted entity {entity:?}");
            commands.run_system_with_input(r_player_systems.setup_player, entity);
        } else {
            info!("Remote character replicated to us: {entity:?}");
        }
        let client_id = connection.id();
        info!(?entity, ?client_id, "Adding physics to character");
        commands.entity(entity).insert(());
    }
}

fn connect_client(mut commands: Commands) {
    commands.connect_client();
}

/// Here we create the lightyear [`ClientPlugins`]
fn build_client_plugin() -> ClientPlugins {
    // Authentication is where you specify how the client should connect to the server
    // This is where you provide the server address.
    let auth = Authentication::Manual {
        server_addr: corp_shared::network::SERVER_ADDR,
        client_id: 42,
        private_key: Key::default(),
        protocol_id: 0,
    };
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the client_address, because we open a UDP socket on the client
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig {
            client_timeout_secs: 3,
            ..default()
        },
    };
    let config = ClientConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        net: net_config,
        ..default()
    };
    ClientPlugins::new(config)
}
