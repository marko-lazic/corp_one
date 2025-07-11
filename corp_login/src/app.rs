use crate::{event::Events, handlers};
use axum::{routing::post, Router};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub events: Events,
}

impl AppState {
    pub async fn new(pool: &SqlitePool, events: &Events) -> Self {
        Self {
            pool: pool.clone(),
            events: events.clone(),
        }
    }
}

pub async fn create_app(app_state: &AppState) -> anyhow::Result<Router> {
    let app = Router::new()
        .route("/register", post(handlers::register_user))
        .route("/login", post(handlers::login_user))
        .route("/validate", post(handlers::validate_token_handler))
        .route("/logout", post(handlers::logout_user))
        .layer(CorsLayer::permissive())
        .with_state(app_state.clone());

    Ok(app)
}
