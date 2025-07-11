use bevy::prelude::*;
use corp_types::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default, Debug)]
pub struct AuthenticatedTokens {
    pub tokens: HashSet<String>,
    pub token_to_user: std::collections::HashMap<String, User>,
}

impl AuthenticatedTokens {
    pub fn add_token(&mut self, token: String, user: User) {
        self.tokens.insert(token.clone());
        self.token_to_user.insert(token, user);
    }

    pub fn remove_token(&mut self, token: &str) {
        self.tokens.remove(token);
        self.token_to_user.remove(token);
    }

    pub fn is_valid(&self, token: &str) -> bool {
        self.tokens.contains(token)
    }

    pub fn get_user(&self, token: &str) -> Option<&User> {
        self.token_to_user.get(token)
    }
}
