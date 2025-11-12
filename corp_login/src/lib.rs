mod app;
pub mod auth;
pub mod config;
pub mod database;
pub mod error;
pub mod handlers;

pub use crate::{
    app::{create_app, AppState},
    config::LoginConfig,
    error::*,
};
use tracing::info;

pub async fn run_login() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = LoginConfig::default();
    run_login_with_config(config).await?;
    Ok(())
}

pub async fn run_login_with_config(config: LoginConfig) -> anyhow::Result<()> {
    let pool = database::setup_database(config.clone()).await?;
    info!("Starting NATS connection: {}", config.nats_url);
    let nc = async_nats::connect(config.nats_url.clone()).await?;
    let app_state = AppState::new(&pool, &nc).await;

    let app = create_app(&app_state).await?;

    let bind_addr = config.bind_address();
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Corp Login server starting on http://{}", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
