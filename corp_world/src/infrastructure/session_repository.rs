use anyhow::Result;
use corp_types::prelude::*;
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct SessionRepository {
    pool: SqlitePool,
}

impl SessionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn colony_to_string(colony: &Option<Colony>) -> Option<String> {
        colony.as_ref().map(|c| match c {
            Colony::StarMap => "StarMap".to_string(),
            Colony::Cloning => "Cloning".to_string(),
            Colony::Iris => "Iris".to_string(),
            Colony::Liberte => "Liberte".to_string(),
            Colony::Playground => "Playground".to_string(),
        })
    }

    fn string_to_colony(s: Option<String>) -> Result<Option<Colony>> {
        match s {
            Some(s) => match s.as_str() {
                "StarMap" => Ok(Some(Colony::StarMap)),
                "Cloning" => Ok(Some(Colony::Cloning)),
                "Iris" => Ok(Some(Colony::Iris)),
                "Liberte" => Ok(Some(Colony::Liberte)),
                "Playground" => Ok(Some(Colony::Playground)),
                _ => Err(anyhow::anyhow!("Invalid colony: {}", s)),
            },
            None => Ok(None),
        }
    }

    fn row_to_session(row: &sqlx::sqlite::SqliteRow) -> Result<PlayerSession> {
        let colony_str: Option<String> = row.try_get("colony")?;

        Ok(PlayerSession {
            id: row.try_get("id")?,
            player_id: row.try_get("player_id")?,
            username: row.try_get("username")?,
            token: row.try_get("token")?,
            colony: Self::string_to_colony(colony_str)?,
            started_at: row.try_get("started_at")?,
            last_heartbeat: row.try_get("last_heartbeat")?,
            is_active: row.try_get("is_active")?,
        })
    }

    pub(crate) async fn create_session(&self, session: PlayerSession) -> Result<PlayerSession> {
        let colony_str = Self::colony_to_string(&session.colony);

        let row = sqlx::query(
            r#"
            INSERT INTO player_sessions (player_id, username, token, colony, started_at, last_heartbeat, is_active)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING id, player_id, username, token, colony, started_at, last_heartbeat, is_active
            "#
        )
        .bind(session.player_id)
        .bind(&session.username)
        .bind(&session.token)
        .bind(colony_str)
        .bind(session.started_at)
        .bind(session.last_heartbeat)
        .bind(session.is_active)
        .fetch_one(&self.pool)
        .await?;

        Self::row_to_session(&row)
    }

    #[allow(dead_code)]
    async fn get_session_by_id(&self, id: i64) -> Result<Option<PlayerSession>> {
        let row = sqlx::query(
            "SELECT id, player_id, username, token, colony, started_at, last_heartbeat, is_active FROM player_sessions WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_session(&row)?)),
            None => Ok(None),
        }
    }

    pub(crate) async fn get_active_session_by_player_id(
        &self,
        player_id: i64,
    ) -> Result<Option<PlayerSession>> {
        let row = sqlx::query(
            "SELECT id, player_id, username, token, colony, started_at, last_heartbeat, is_active FROM player_sessions WHERE player_id = ? AND is_active = 1 ORDER BY started_at DESC LIMIT 1"
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_session(&row)?)),
            None => Ok(None),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn get_active_session_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PlayerSession>> {
        let row = sqlx::query(
            "SELECT id, player_id, username, token, colony, started_at, last_heartbeat, is_active FROM player_sessions WHERE username = ? AND is_active = 1 ORDER BY started_at DESC LIMIT 1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_session(&row)?)),
            None => Ok(None),
        }
    }

    pub(crate) async fn get_active_session_by_token(
        &self,
        token: &str,
    ) -> Result<Option<PlayerSession>> {
        let row = sqlx::query(
            "SELECT id, player_id, username, token, colony, started_at, last_heartbeat, is_active FROM player_sessions WHERE token = ? AND is_active = 1 ORDER BY started_at DESC LIMIT 1"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_session(&row)?)),
            None => Ok(None),
        }
    }

    #[allow(dead_code)]
    async fn update_session(&self, session: &PlayerSession) -> Result<()> {
        let colony_str = Self::colony_to_string(&session.colony);

        sqlx::query(
            r#"
            UPDATE player_sessions 
            SET colony = ?, last_heartbeat = ?, is_active = ?
            WHERE id = ?
            "#,
        )
        .bind(colony_str)
        .bind(session.last_heartbeat)
        .bind(session.is_active)
        .bind(session.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub(crate) async fn end_session(&self, session_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE player_sessions SET is_active = 0, last_heartbeat = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) async fn update_heartbeat(&self, session_id: i64) -> Result<()> {
        sqlx::query("UPDATE player_sessions SET last_heartbeat = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) async fn set_colony(&self, session_id: i64, colony: Option<Colony>) -> Result<()> {
        let colony_str = Self::colony_to_string(&colony);

        sqlx::query(
            "UPDATE player_sessions SET colony = ?, last_heartbeat = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(colony_str)
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) async fn get_active_sessions(&self) -> Result<Vec<PlayerSession>> {
        let rows = sqlx::query(
            "SELECT id, player_id, username, token, colony, started_at, last_heartbeat, is_active FROM player_sessions WHERE is_active = 1 ORDER BY started_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(Self::row_to_session(&row)?);
        }

        Ok(sessions)
    }

    #[allow(dead_code)]
    async fn get_expired_sessions(&self, timeout_minutes: i64) -> Result<Vec<PlayerSession>> {
        let rows = sqlx::query(
            r#"
            SELECT id, player_id, username, token, colony, started_at, last_heartbeat, is_active 
            FROM player_sessions 
            WHERE is_active = 1 
            AND datetime(last_heartbeat, '+' || ? || ' minutes') < datetime('now')
            ORDER BY last_heartbeat ASC
            "#,
        )
        .bind(timeout_minutes)
        .fetch_all(&self.pool)
        .await?;

        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(Self::row_to_session(&row)?);
        }

        Ok(sessions)
    }

    #[allow(dead_code)]
    pub(crate) async fn cleanup_expired_sessions(&self, timeout_minutes: i64) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE player_sessions 
            SET is_active = 0, last_heartbeat = CURRENT_TIMESTAMP
            WHERE is_active = 1 
            AND datetime(last_heartbeat, '+' || ? || ' minutes') < datetime('now')
            "#,
        )
        .bind(timeout_minutes)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
