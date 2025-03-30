use crate::{game::prelude::*, log::init_logging};
use corp_shared::prelude::Colony;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod dirs;
mod game;
mod log;
mod table;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_logging()?;
    let config = ServerConfig {
        colony: Colony::Iris,
        server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000),
    };
    let _iris_ref = kameo::actor::spawn_in_thread(GameInstanceActor { config });
    let config = ServerConfig {
        colony: Colony::Cloning,
        server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5001),
    };
    let _cloning_ref = kameo::actor::spawn_in_thread(GameInstanceActor { config });

    println!("Actors running. Press CTRL+C to stop.");
    tokio::signal::ctrl_c().await?;
    println!("Shutting down.");
    Ok(())
}
