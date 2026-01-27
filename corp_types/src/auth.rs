pub use auth_types::*;
pub use events::*;

pub mod auth_types {
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

    #[derive(Debug, Serialize, Deserialize)]
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
}

pub mod events {
    use crate::auth::auth_types::{Token, User};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum UserEvent {
        Register(User),
        Login(TokenUserData),
        Logout(TokenData),
        TokenExpired(TokenData),
    }

    impl UserEvent {
        pub fn register(user: &User) -> Self {
            Self::Register(user.clone())
        }
        pub fn login(user: &User, token: &Token) -> Self {
            Self::Login(TokenUserData {
                token: token.clone(),
                user: user.clone(),
            })
        }

        pub fn logout(token: impl Into<String>) -> Self {
            Self::Logout(TokenData {
                token: token.into(),
            })
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TokenUserData {
        pub token: Token,
        pub user: User,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TokenData {
        pub token: String,
    }
}
