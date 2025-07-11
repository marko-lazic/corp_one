use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationEvent {
    pub event_type: AuthEventType,
    pub timestamp: DateTime<Utc>,
}

impl AuthenticationEvent {
    pub fn login_event(user: &User, token: &Token) -> Self {
        Self {
            event_type: AuthEventType::Login(TokenUserData {
                token: token.clone(),
                user: user.clone(),
            }),
            timestamp: Utc::now(),
        }
    }

    pub fn logout_event(token: impl Into<String>) -> Self {
        Self {
            event_type: AuthEventType::Logout(TokenData {
                token: token.into(),
            }),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthEventType {
    Login(TokenUserData),
    Logout(TokenData),
    TokenExpired(TokenData),
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
