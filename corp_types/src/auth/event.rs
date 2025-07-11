use crate::{Token, User};
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
            event_type: AuthEventType::Login(LoginEventData {
                token: token.clone(),
                user: user.clone(),
            }),
            timestamp: Utc::now(),
        }
    }

    pub fn logout_event(token: impl Into<String>) -> Self {
        Self {
            event_type: AuthEventType::Logout(LogoutEventData {
                token: token.into(),
            }),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthEventType {
    Login(LoginEventData),
    Logout(LogoutEventData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginEventData {
    pub token: Token,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutEventData {
    pub token: String,
}
