use anyhow::Result;
use corp_world::run_world;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    corp_shared::prelude::init_logging()?;
    info!("Starting corp_world...");

    run_world().await?;

    info!("All actors started successfully. Press CTRL+C to stop.");
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, stopping actors...");
    info!("Server shutdown complete.");
    Ok(())
}
