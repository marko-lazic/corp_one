use crate::{
    app::AppState,
    auth::{authenticate_user_and_generate_token, create_user, invalidate_token, validate_token},
};
use anyhow::Result;
use axum::{extract::State, response::Json};
use corp_types::{
    ApiError, AuthenticationEvent, CreateUserRequest, LoginRequest, LoginResponse, LogoutRequest,
    User, ValidateTokenRequest, ValidateTokenResponse,
};
use tracing::{error, info};

pub async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    info!("Registration attempt for user: {}", request.username);

    if request.username.is_empty() || request.email.is_empty() || request.password.is_empty() {
        return Err(ApiError::missing_fields());
    }

    if request.password.len() < 6 {
        return Err(ApiError::weak_password());
    }

    match create_user(&state.pool, request).await {
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
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    info!("Login attempt for user: {}", request.username);

    if request.username.is_empty() || request.password.is_empty() {
        return Err(ApiError::missing_fields());
    }

    match authenticate_user_and_generate_token(&state.pool, request).await {
        Ok(Some((user, token))) => {
            info!("User logged in successfully: {}", user.username);

            state
                .events
                .send(AuthenticationEvent::login_event(&user, &token));

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
    State(state): State<AppState>,
    Json(request): Json<ValidateTokenRequest>,
) -> Result<Json<ValidateTokenResponse>, ApiError> {
    info!("Token validation request");

    if request.token.is_empty() {
        return Err(ApiError::missing_token());
    }

    match validate_token(&state.pool, &request.token).await {
        Ok(Some((user, token))) => {
            info!("Token validated successfully for user: {}", user.username);
            let response = ValidateTokenResponse { user, token };
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
    State(state): State<AppState>,
    Json(request): Json<LogoutRequest>,
) -> Result<Json<String>, ApiError> {
    info!("Logout request");

    if request.token.is_empty() {
        return Err(ApiError::missing_token());
    }

    match invalidate_token(&state.pool, &request.token).await {
        Ok(true) => {
            info!("Token invalidated successfully");

            state
                .events
                .send(AuthenticationEvent::logout_event(&request.token));
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
