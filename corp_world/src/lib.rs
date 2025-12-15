pub mod application;
pub mod config;
pub mod infrastructure;

pub use crate::config::WorldConfig;
pub use application::*;
use corp_types::messenger::Messenger;
pub use infrastructure::*;
use kameo::prelude::*;
use tracing::info;

pub struct WorldContext {
    pub player_repo: PlayerRepository,
    pub player_service: ActorRef<PlayerServiceActor>,
    pub session_repo: SessionRepository,
    pub session_service: ActorRef<SessionServiceActor>,
}

pub async fn run_world() -> anyhow::Result<WorldContext> {
    dotenvy::dotenv().ok();
    let config = WorldConfig::default();
    run_world_with_config(config).await
}

pub async fn run_world_with_config(config: WorldConfig) -> anyhow::Result<WorldContext> {
    let pool = setup_database(config.clone()).await?;
    info!("World database setup completed");

    info!("Connecting to NATS server at {}", config.nats_url);
    let nc = async_nats::connect(&config.nats_url).await?;
    let messenger = Messenger::new(nc);

    let player_repo = PlayerRepository::new(pool.clone());
    let player_service =
        PlayerServiceActor::spawn(PlayerServiceActor::new(&player_repo, &messenger));

    let session_repo = SessionRepository::new(pool.clone());
    let session_service = SessionServiceActor::spawn(SessionServiceActor::new(
        &session_repo,
        &player_repo,
        &messenger,
    ));

    info!("NATS event handlers set up successfully");

    Ok(WorldContext {
        player_repo,
        player_service,
        session_repo,
        session_service,
    })
}
