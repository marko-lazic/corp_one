use crate::prelude::*;
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
