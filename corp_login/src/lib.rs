pub mod auth;
pub mod database;
pub mod handlers;

use anyhow::Result;
use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;
use tracing::log;

pub async fn create_app() -> Result<Router> {
    let pool = database::setup_database().await?;

    let app = Router::new()
        .route("/register", post(handlers::register_user))
        .route("/login", post(handlers::login_user))
        .route("/validate", post(handlers::validate_token_handler))
        .route("/logout", post(handlers::logout_user))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    Ok(app)
}

pub async fn run_server() -> Result<()> {
    let app = create_app().await?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:25560").await?;
    tracing::info!("Corp Login server starting on http://127.0.0.1:25560");

    axum::serve(listener, app).await?;

    Ok(())
}
