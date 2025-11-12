use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};
use corp_types::messenger::Messenger;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    messenger: Messenger,
}

impl AppState {
    pub async fn new(pool: &SqlitePool, nc: &async_nats::Client) -> Self {
        Self {
            pool: pool.clone(),
            messenger: Messenger::new(nc.clone()),
        }
    }

    pub fn message(&self) -> &Messenger {
        &self.messenger
    }
}

pub async fn create_app(app_state: &AppState) -> anyhow::Result<Router> {
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/register", post(handlers::register_user))
        .route("/login", post(handlers::login_user))
        .route("/validate", post(handlers::validate_token_handler))
        .route("/logout", post(handlers::logout_user))
        .layer(CorsLayer::permissive())
        .with_state(app_state.clone());

    Ok(app)
}
