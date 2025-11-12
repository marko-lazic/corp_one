use anyhow::Result;
use corp_login::run_login;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    run_login().await?;
    Ok(())
}
