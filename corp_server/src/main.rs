use crate::{login::LoginActor, proxy::ProxyActor, server::*};
use aeronet_webtransport::wtransport::Identity;
use bevy::ecs::error::{warn, GLOBAL_ERROR_HANDLER};
use corp_shared::prelude::*;
use game::GameServerActor;
use kameo::Actor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::info;

mod dirs;
mod game;
pub mod login;
mod proxy;
mod server;
mod table;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_logging()?;
    GLOBAL_ERROR_HANDLER
        .set(warn)
        .expect("The error handler can only be set once, globally.");
    let identity = Identity::load_pemfiles("./certs/server.pem", "./certs/server.key").await?;

    let iris_ref = GameServerActor::spawn_in_thread(GameServerActor {
        config: ColonyAppConfig {
            colony: Colony::Iris,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
            identity: identity.clone_identity(),
        },
    });

    let cloning_ref = GameServerActor::spawn_in_thread(GameServerActor {
        config: ColonyAppConfig {
            colony: Colony::Cloning,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
            identity: identity.clone_identity(),
        },
    });

    let star_map_ref = GameServerActor::spawn_in_thread(GameServerActor {
        config: ColonyAppConfig {
            colony: Colony::StarMap,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25567),
            identity: identity.clone_identity(),
        },
    });

    let liberte_ref = GameServerActor::spawn_in_thread(GameServerActor {
        config: ColonyAppConfig {
            colony: Colony::Liberte,
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25568),
            identity: identity.clone_identity(),
        },
    });

    let _proxy_ref = ProxyActor::spawn_in_thread(ProxyActor);

    let login_ref = LoginActor::spawn_in_thread(LoginActor::new());

    // Register actors in the global registry for better discovery
    login_ref.register("login")?;
    iris_ref.register("iris")?;
    cloning_ref.register("cloning")?;
    star_map_ref.register("starmap")?;
    liberte_ref.register("liberte")?;

    // Add supervision links between login actor and game servers
    login_ref.link(&iris_ref).await;
    login_ref.link(&cloning_ref).await;
    login_ref.link(&star_map_ref).await;
    login_ref.link(&liberte_ref).await;

    // Game servers are now automatically discovered via registry
    // No manual registration needed - LoginActor will find them by name

    info!("All actors started successfully. Press CTRL+C to stop.");

    // Graceful shutdown handling
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, stopping actors...");

    // Actors will be automatically stopped when they go out of scope
    // The supervision links ensure proper cleanup
    info!("Server shutdown complete.");
    Ok(())
}
