use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

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
