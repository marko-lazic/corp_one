use crate::{config::GameServerConfig, game::GameServer};
use aeronet_webtransport::wtransport::Identity;
use bevy::ecs::error::{warn, GLOBAL_ERROR_HANDLER};
use corp_shared::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::{error, info};

mod channel;
mod config;
mod game;
pub mod integration;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_logging()?;
    info!("Starting Corp Server...");
    GLOBAL_ERROR_HANDLER
        .set(warn)
        .expect("The error handler can only be set once, globally.");
    let identity = Identity::load_pemfiles("./certs/server.pem", "./certs/server.key").await?;

    tokio::spawn(async move {
        if let Err(e) = corp_proxy::init::init().await {
            tracing::error!("Proxy service error: {}", e);
        }
    });
    tokio::spawn(async move {
        if let Err(e) = corp_login::run_login().await {
            tracing::error!("Login service error: {}", e);
        }
    });

    let game_server_configs = vec![
        GameServerConfig {
            colony: Colony::Iris,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
            identity: identity.clone_identity(),
        },
        GameServerConfig {
            colony: Colony::Cloning,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
            identity: identity.clone_identity(),
        },
        GameServerConfig {
            colony: Colony::StarMap,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25567),
            identity: identity.clone_identity(),
        },
        GameServerConfig {
            colony: Colony::Liberte,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25568),
            identity: identity.clone_identity(),
        },
    ];

    info!("Starting game servers...");
    for config in game_server_configs {
        let game_server_handle = std::thread::Builder::new()
            .name(format!("game_server_{}", config.colony.to_snake_case()))
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                let server = GameServer::new(config);
                server.run();
            });

        if let Err(e) = game_server_handle {
            error!("Failed to start game server thread: {:?}", e);
        }
    }

    corp_world::run_world().await?;

    info!("All actors started successfully. Press CTRL+C to stop.");
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, stopping actors...");
    info!("Server shutdown complete.");
    Ok(())
}
