use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PlayerSession {
    pub id: i64,
    pub player_id: i64,
    pub username: String,
    pub token: Option<String>,
    pub colony: Option<Colony>,
    pub started_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub is_active: bool,
}

impl PlayerSession {
    pub fn new(player_id: i64, username: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Will be set by database
            player_id,
            username,
            token: None,
            colony: None, // Will be set later by corp_server
            started_at: now,
            last_heartbeat: now,
            is_active: true,
        }
    }

    pub fn new_with_token(player_id: i64, username: String, token: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Will be set by database
            player_id,
            username,
            token: Some(token),
            colony: None, // Will be set later by corp_server
            started_at: now,
            last_heartbeat: now,
            is_active: true,
        }
    }

    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = Utc::now();
    }

    pub fn set_colony(&mut self, colony: Option<Colony>) {
        self.colony = colony;
        self.last_heartbeat = Utc::now();
    }

    pub fn end_session(&mut self) {
        self.is_active = false;
        self.last_heartbeat = Utc::now();
    }

    pub fn is_expired(&self, timeout_minutes: i64) -> bool {
        let now = Utc::now();
        let timeout_duration = chrono::Duration::minutes(timeout_minutes);
        now.signed_duration_since(self.last_heartbeat) > timeout_duration
    }
}
