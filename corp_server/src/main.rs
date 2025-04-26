use crate::{game::*, proxy::ProxyActor};
use aeronet_webtransport::wtransport::Identity;
use corp_shared::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod dirs;
mod game;
mod proxy;
mod table;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_logging()?;
    let identity = Identity::load_pemfiles("./certs/server.pem", "./certs/server.key").await?;

    let config = GameInstanceConfig {
        colony: Colony::Iris,
        server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
        identity: identity.clone_identity(),
    };
    let _iris_ref = kameo::actor::spawn_in_thread(GameInstanceActor { config });

    let config = GameInstanceConfig {
        colony: Colony::Cloning,
        server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
        identity,
    };
    let _cloning_ref = kameo::actor::spawn_in_thread(GameInstanceActor { config });

    let _proxy_ref = kameo::actor::spawn_in_thread(ProxyActor);

    println!("Actors running. Press CTRL+C to stop.");
    tokio::signal::ctrl_c().await?;
    println!("Shutting down.");
    Ok(())
}
