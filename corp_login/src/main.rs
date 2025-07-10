use anyhow::Result;
use corp_login::run_server;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    run_server().await?;
    Ok(())
}
