use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Player {
    player_id: i64,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
    level: i32,
    experience: i64,
    coin: i64,
    last_login: Option<DateTime<Utc>>,
}
