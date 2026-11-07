use anyhow::Result;
use bevy::log::{tracing, tracing_subscriber};

pub fn init_logging() -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new(
            "warn,corp_server=info,corp_proxy=info,corp_login=info,corp_world=info,corp_login::database=warn,corp_world::infrastructure::database=warn",
        )
    });
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
