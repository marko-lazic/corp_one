use crate::auth::{authenticate_user_and_generate_token, create_user, validate_token, invalidate_token};
use corp_types::{ApiError, CreateUserRequest, LoginRequest, LoginResponse, User, ValidateTokenRequest, ValidateTokenResponse, LogoutRequest};
use anyhow::Result;
use axum::{
    extract::State,
    response::Json,
};
use sqlx::SqlitePool;
use tracing::{error, info};

pub async fn register_user(
    State(pool): State<SqlitePool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    info!("Registration attempt for user: {}", request.username);

    if request.username.is_empty() || request.email.is_empty() || request.password.is_empty() {
        return Err(ApiError::missing_fields());
    }

    if request.password.len() < 6 {
        return Err(ApiError::weak_password());
    }

    match create_user(&pool, request).await {
        Ok(user) => {
            info!("User registered successfully: {}", user.username);
            Ok(Json(user))
        }
        Err(e) => {
            error!("Registration failed: {}", e);
            if e.to_string().contains("UNIQUE constraint failed") {
                Err(ApiError::user_exists())
            } else {
                Err(ApiError::internal_error())
            }
        }
    }
}

pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    info!("Login attempt for user: {}", request.username);

    if request.username.is_empty() || request.password.is_empty() {
        return Err(ApiError::missing_fields());
    }

    match authenticate_user_and_generate_token(&pool, request).await {
        Ok(Some((user, token))) => {
            info!("User logged in successfully: {}", user.username);
            let response = LoginResponse {
                token: token.token,
                expires_at: token.expires_at,
                user,
            };
            Ok(Json(response))
        }
        Ok(None) => {
            error!("Invalid credentials");
            Err(ApiError::invalid_credentials())
        }
        Err(e) => {
            error!("Login failed: {}", e);
            Err(ApiError::internal_error())
        }
    }
}

pub async fn validate_token_handler(
    State(pool): State<SqlitePool>,
    Json(request): Json<ValidateTokenRequest>,
) -> Result<Json<ValidateTokenResponse>, ApiError> {
    info!("Token validation request");

    if request.token.is_empty() {
        return Err(ApiError::missing_token());
    }

    match validate_token(&pool, &request.token).await {
        Ok(Some((user, token))) => {
            info!("Token validated successfully for user: {}", user.username);
            let response = ValidateTokenResponse {
                user,
                token,
            };
            Ok(Json(response))
        }
        Ok(None) => {
            error!("Token validation failed");
            Err(ApiError::invalid_token())
        }
        Err(e) => {
            error!("Token validation failed: {}", e);
            Err(ApiError::internal_error())
        }
    }
}

pub async fn logout_user(
    State(pool): State<SqlitePool>,
    Json(request): Json<LogoutRequest>,
) -> Result<Json<String>, ApiError> {
    info!("Logout request");

    if request.token.is_empty() {
        return Err(ApiError::missing_token());
    }

    match invalidate_token(&pool, &request.token).await {
        Ok(true) => {
            info!("Token invalidated successfully");
            Ok(Json("Logged out".to_string()))
        }
        Ok(false) => {
            error!("Token not found for invalidation");
            Err(ApiError::token_not_found())
        }
        Err(e) => {
            error!("Logout failed: {}", e);
            Err(ApiError::internal_error())
        }
    }
}
