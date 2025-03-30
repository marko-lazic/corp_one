use crate::game::{config::ServerConfig, database::DbPlugin, server::ServerNetPlugin};
use bevy::{app::ScheduleRunnerPlugin, prelude::*, state::app::StatesPlugin};
use bevy_rand::prelude::*;
use corp_shared::network::shared_config;
use lightyear::{
    connection::server::{IoConfig, NetConfig},
    prelude::server::{NetcodeConfig, ServerPlugins, ServerTransport},
};
use std::{net::SocketAddr, time::Duration};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ServerState {
    #[default]
    Load,
    Serve,
}

pub fn new_app(config: ServerConfig) {
    let frames_per_second = Duration::from_secs_f32(1.0 / 60.0);

    App::new()
        .insert_resource(config)
        .add_plugins((
            build_server_plugin(config.server_addr),
            StatesPlugin,
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frames_per_second)),
            DbPlugin,
            ServerNetPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .init_state::<ServerState>()
        .run();
}

/// Here we create the lightyear [`ServerPlugins`]
fn build_server_plugin(server_addr: SocketAddr) -> ServerPlugins {
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the server_address, because we open a UDP socket on the server
        transport: ServerTransport::UdpSocket(server_addr),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };
    let config = lightyear::prelude::server::ServerConfig {
        shared: shared_config(),
        net: vec![net_config],
        ..default()
    };
    ServerPlugins::new(config)
}
