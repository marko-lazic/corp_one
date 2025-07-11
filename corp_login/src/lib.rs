mod app;
pub mod auth;
pub mod database;
pub mod dirs;
pub mod event;
pub mod handlers;

pub use crate::{
    app::{create_app, AppState},
    event::Events,
};
use anyhow::Result;
use axum::{routing::post, Router};
pub use event::*;
use tower_http::cors::CorsLayer;
use tracing::log;

pub async fn run_server(events: &Events) -> Result<()> {
    let pool = database::setup_database().await?;
    let app_state = AppState::new(&pool, &events).await;

    let app = create_app(&app_state).await?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:25550").await?;
    tracing::info!("Corp Login server starting on http://127.0.0.1:25550");

    axum::serve(listener, app).await?;
    Ok(())
}
