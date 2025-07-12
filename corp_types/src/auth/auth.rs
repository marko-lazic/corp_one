use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateTokenResponse {
    pub user: User,
    pub token: Token,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(status: u16, code: &str, message: &str) -> Self {
        Self {
            status,
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn invalid_credentials() -> Self {
        Self::new(401, "INVALID_CREDENTIALS", "Invalid username or password")
    }

    pub fn invalid_token() -> Self {
        Self::new(401, "INVALID_TOKEN", "Token is invalid or expired")
    }

    pub fn missing_token() -> Self {
        Self::new(400, "MISSING_TOKEN", "Token is required")
    }

    pub fn token_not_found() -> Self {
        Self::new(404, "TOKEN_NOT_FOUND", "Token not found")
    }

    pub fn missing_fields() -> Self {
        Self::new(400, "MISSING_FIELDS", "Missing required fields")
    }

    pub fn weak_password() -> Self {
        Self::new(
            400,
            "WEAK_PASSWORD",
            "Password must be at least 6 characters long",
        )
    }

    pub fn user_exists() -> Self {
        Self::new(409, "USER_EXISTS", "Username or email already exists")
    }

    pub fn internal_error() -> Self {
        Self::new(500, "INTERNAL_ERROR", "Internal server error")
    }

    pub fn forbidden() -> Self {
        Self::new(403, "FORBIDDEN", "Access forbidden")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}
