use anyhow::Result;
use corp_types::prelude::*;
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct PlayerRepository {
    pool: SqlitePool,
}

impl PlayerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn faction_to_string(faction: &Faction) -> String {
        match faction {
            Faction::EC => "EC".to_string(),
            Faction::CMG => "CMG".to_string(),
            Faction::VI => "VI".to_string(),
        }
    }

    fn string_to_faction(s: &str) -> Result<Faction> {
        match s {
            "EC" => Ok(Faction::EC),
            "CMG" => Ok(Faction::CMG),
            "VI" => Ok(Faction::VI),
            _ => Err(anyhow::anyhow!("Invalid faction: {}", s)),
        }
    }

    fn rank_to_string(rank: &Rank) -> String {
        match rank {
            Rank::R0 => "R0".to_string(),
            Rank::R1 => "R1".to_string(),
            Rank::R2 => "R2".to_string(),
            Rank::R3 => "R3".to_string(),
            Rank::R4 => "R4".to_string(),
            Rank::R5 => "R5".to_string(),
            Rank::R6 => "R6".to_string(),
            Rank::R7 => "R7".to_string(),
        }
    }

    fn string_to_rank(s: &str) -> Result<Rank> {
        match s {
            "R0" => Ok(Rank::R0),
            "R1" => Ok(Rank::R1),
            "R2" => Ok(Rank::R2),
            "R3" => Ok(Rank::R3),
            "R4" => Ok(Rank::R4),
            "R5" => Ok(Rank::R5),
            "R6" => Ok(Rank::R6),
            "R7" => Ok(Rank::R7),
            _ => Err(anyhow::anyhow!("Invalid rank: {}", s)),
        }
    }

    fn row_to_player(row: &sqlx::sqlite::SqliteRow) -> Result<Player> {
        let faction_str: String = row.try_get("faction")?;
        let rank_str: String = row.try_get("rank")?;

        Ok(Player {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            character_name: row.try_get("character_name")?,
            current_health: row.try_get("current_health")?,
            credits: row.try_get("credits")?,
            faction: Self::string_to_faction(&faction_str)?,
            rank: Self::string_to_rank(&rank_str)?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    pub(crate) async fn create_player(&self, player: Player) -> Result<Player> {
        let faction_str = Self::faction_to_string(&player.faction);
        let rank_str = Self::rank_to_string(&player.rank);

        let row = sqlx::query(
            r#"
            INSERT INTO players (username, character_name, current_health, credits, faction, rank, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id, username, character_name, current_health, credits, faction, rank, created_at, updated_at
            "#
        )
        .bind(&player.username)
        .bind(&player.character_name)
        .bind(player.current_health)
        .bind(player.credits)
        .bind(&faction_str)
        .bind(&rank_str)
        .bind(player.created_at)
        .bind(player.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Self::row_to_player(&row)
    }

    #[allow(dead_code)]
    async fn get_player_by_id(&self, id: i64) -> Result<Option<Player>> {
        let row = sqlx::query(
            "SELECT id, username, character_name, current_health, credits, faction, rank, created_at, updated_at FROM players WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_player(&row)?)),
            None => Ok(None),
        }
    }

    pub(crate) async fn get_player_by_username(&self, username: &str) -> Result<Option<Player>> {
        let row = sqlx::query(
            "SELECT id, username, character_name, current_health, credits, faction, rank, created_at, updated_at FROM players WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_player(&row)?)),
            None => Ok(None),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn update_player(&self, player: &Player) -> Result<()> {
        let faction_str = Self::faction_to_string(&player.faction);
        let rank_str = Self::rank_to_string(&player.rank);

        sqlx::query(
            r#"
            UPDATE players 
            SET character_name = ?, current_health = ?, credits = ?, faction = ?, rank = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&player.character_name)
        .bind(player.current_health)
        .bind(player.credits)
        .bind(&faction_str)
        .bind(&rank_str)
        .bind(player.updated_at)
        .bind(player.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    async fn delete_player(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM players WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    async fn list_players(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Player>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query(
            "SELECT id, username, character_name, current_health, credits, faction, rank, created_at, updated_at FROM players ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut players = Vec::new();
        for row in rows {
            players.push(Self::row_to_player(&row)?);
        }

        Ok(players)
    }

    pub(crate) async fn player_exists(&self, username: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM players WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        Ok(count > 0)
    }
}
