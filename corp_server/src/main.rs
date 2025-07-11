use crate::{login::LoginActor, proxy::ProxyActor, server::*};
use aeronet_webtransport::wtransport::Identity;
use bevy::ecs::error::{warn, GLOBAL_ERROR_HANDLER};
use corp_shared::prelude::*;
use corp_types::prelude::*;
use game::GameServerActor;
use kameo::Actor;
use kameo_actors::pubsub::{PubSub, Subscribe};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::info;

mod config;
mod game;
pub mod login;
mod proxy;
pub mod server;
mod table;
mod token;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_logging()?;
    GLOBAL_ERROR_HANDLER
        .set(warn)
        .expect("The error handler can only be set once, globally.");
    let identity = Identity::load_pemfiles("./certs/server.pem", "./certs/server.key").await?;

    let auth_pub_sub_ref = PubSub::spawn(PubSub::<AuthenticationEvent>::new());
    auth_pub_sub_ref.register("auth_pub_sub")?;

    let proxy_ref = ProxyActor::spawn(ProxyActor);
    proxy_ref.register("proxy")?;
    let login_ref = LoginActor::spawn(LoginActor::new(auth_pub_sub_ref.clone()));
    login_ref.register("login")?;

    let tokens_ref = Tokens::spawn(Tokens::new());
    tokens_ref.register("tokens")?;
    auth_pub_sub_ref.ask(Subscribe(tokens_ref.clone())).await?;

    let game_server_configs = vec![
        GameServerConfig {
            colony: Colony::Iris,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
            identity: identity.clone_identity(),
            tokens_ref: tokens_ref.clone(),
        },
        GameServerConfig {
            colony: Colony::Cloning,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
            identity: identity.clone_identity(),
            tokens_ref: tokens_ref.clone(),
        },
        GameServerConfig {
            colony: Colony::StarMap,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25567),
            identity: identity.clone_identity(),
            tokens_ref: tokens_ref.clone(),
        },
        GameServerConfig {
            colony: Colony::Liberte,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25568),
            identity: identity.clone_identity(),
            tokens_ref: tokens_ref.clone(),
        },
    ];

    for config in game_server_configs {
        let game_server_ref = GameServerActor::spawn_in_thread(GameServerActor::new(&config));
        game_server_ref.register(config.colony.to_string().to_lowercase())?;
    }

    info!("All actors started successfully. Press CTRL+C to stop.");
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, stopping actors...");
    info!("Server shutdown complete.");
    Ok(())
}
