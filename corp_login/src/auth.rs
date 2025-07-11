use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use corp_types::prelude::*;
use sqlx::SqlitePool;
use tracing::{error, info};
use uuid::Uuid;

pub fn hash_password(password: &str) -> Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}

pub fn generate_token() -> String {
    Uuid::new_v4().to_string()
}

pub async fn create_user(pool: &SqlitePool, request: CreateUserRequest) -> Result<User> {
    let hashed_password = hash_password(&request.password)?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password)
        VALUES (?, ?, ?)
        RETURNING id, username, email, password, created_at, updated_at
        "#,
    )
    .bind(&request.username)
    .bind(&request.email)
    .bind(&hashed_password)
    .fetch_one(pool)
    .await?;

    info!("User created successfully: {}", user.username);
    Ok(user)
}

pub async fn authenticate_user_and_generate_token(
    pool: &SqlitePool,
    request: LoginRequest,
) -> Result<Option<(User, Token)>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password, created_at, updated_at FROM users WHERE username = ?"
    )
    .bind(&request.username)
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user {
        if verify_password(&request.password, &user.password)? {
            info!("User authenticated successfully: {}", user.username);

            // Invalidate all existing tokens for this user
            invalidate_user_tokens(pool, user.id).await?;

            // Generate new token
            let token = create_token_for_user(pool, user.id).await?;

            return Ok(Some((user, token)));
        }
    }

    error!("Authentication failed for user: {}", request.username);
    Ok(None)
}

pub async fn create_token_for_user(pool: &SqlitePool, user_id: i64) -> Result<Token> {
    let token_str = generate_token();
    let expires_at = Utc::now() + Duration::hours(24); // Token expires in 24 hours

    let token = sqlx::query_as::<_, Token>(
        r#"
        INSERT INTO tokens (user_id, token, expires_at)
        VALUES (?, ?, ?)
        RETURNING id, user_id, token, created_at, expires_at
        "#,
    )
    .bind(user_id)
    .bind(&token_str)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    info!("Token created for user_id: {}", user_id);
    Ok(token)
}

pub async fn validate_token(pool: &SqlitePool, token_str: &str) -> Result<Option<(User, Token)>> {
    // First clean up expired tokens
    clean_expired_tokens(pool).await?;

    // Get the token first
    let token = sqlx::query_as::<_, Token>(
        "SELECT id, user_id, token, created_at, expires_at FROM tokens WHERE token = ? AND expires_at > datetime('now')"
    )
    .bind(token_str)
    .fetch_optional(pool)
    .await?;

    if let Some(token) = token {
        // Get the user for this token
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password, created_at, updated_at FROM users WHERE id = ?",
        )
        .bind(token.user_id)
        .fetch_one(pool)
        .await?;

        info!("Token validated successfully for user: {}", user.username);
        Ok(Some((user, token)))
    } else {
        error!("Token validation failed for token: {}", token_str);
        Ok(None)
    }
}

pub async fn invalidate_token(pool: &SqlitePool, token_str: &str) -> Result<bool> {
    let result = sqlx::query("DELETE FROM tokens WHERE token = ?")
        .bind(token_str)
        .execute(pool)
        .await?;

    let invalidated = result.rows_affected() > 0;
    if invalidated {
        info!("Token invalidated successfully");
    } else {
        error!("Token not found for invalidation");
    }

    Ok(invalidated)
}

pub async fn invalidate_user_tokens(pool: &SqlitePool, user_id: i64) -> Result<u64> {
    let result = sqlx::query("DELETE FROM tokens WHERE user_id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;

    let count = result.rows_affected();
    info!("Invalidated {} tokens for user_id: {}", count, user_id);
    Ok(count)
}

pub async fn clean_expired_tokens(pool: &SqlitePool) -> Result<u64> {
    let result = sqlx::query("DELETE FROM tokens WHERE expires_at <= datetime('now')")
        .execute(pool)
        .await?;

    let count = result.rows_affected();
    if count > 0 {
        info!("Cleaned up {} expired tokens", count);
    }
    Ok(count)
}

pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password, created_at, updated_at FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password, created_at, updated_at FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
