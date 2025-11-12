use crate::prelude::*;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub character_name: String,
    pub current_health: i32,
    pub credits: i64,
    pub faction: Faction,
    pub rank: Rank,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Player {
    pub fn new(username: String, character_name: String, faction: Faction) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            username,
            character_name,
            current_health: 100,
            credits: 1000,
            faction,
            rank: Rank::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.current_health = (self.current_health - damage).max(0);
        self.updated_at = Utc::now();
    }

    pub fn heal(&mut self, amount: i32) {
        self.current_health = (self.current_health + amount).min(100);
        self.updated_at = Utc::now();
    }

    pub fn add_credits(&mut self, amount: i64) {
        self.credits += amount;
        self.updated_at = Utc::now();
    }

    pub fn spend_credits(&mut self, amount: i64) -> anyhow::Result<()> {
        if self.credits >= amount {
            self.credits -= amount;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(anyhow!("Insufficient credits"))
        }
    }
}
