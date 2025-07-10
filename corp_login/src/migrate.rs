use anyhow::Result;
use corp_login::database;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> Result<()> {
    init();

    println!("Running database migrations...");
    database::setup_database().await?;
    println!("Migration completed successfully!");

    Ok(())
}
