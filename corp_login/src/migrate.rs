use anyhow::Result;
use corp_login::{database::setup_database, LoginConfig};
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    dotenvy::dotenv().ok();
    let config = LoginConfig::default();
    println!("Running database migrations...");
    setup_database(config).await?;
    println!("Migration completed successfully!");

    Ok(())
}
