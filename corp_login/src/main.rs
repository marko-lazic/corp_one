use crate::event::Events;
use anyhow::Result;
use corp_login::run_server;
use tracing_subscriber::fmt::init;

mod event;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let events = Events::new();
    run_server(&events).await?;
    Ok(())
}
