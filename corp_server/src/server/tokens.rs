use bevy::prelude::*;
use corp_types::auth::{User, UserEvent};
use std::collections::HashSet;

#[derive(Event)]
pub struct BevyUserEvent(pub UserEvent);

#[derive(Resource, Default, Debug)]
pub struct Tokens {
    pub tokens: HashSet<String>,
    pub token_to_user: std::collections::HashMap<String, User>,
}

impl Tokens {
    pub fn new() -> Self {
        Self::default()
    }
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

pub struct TokensPlugin;

impl Plugin for TokensPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tokens>()
            .add_observer(on_bevy_user_event);
    }
}

fn on_bevy_user_event(trigger: Trigger<BevyUserEvent>, mut tokens: ResMut<Tokens>) {
    let user_event = trigger.event().0.clone();
    info!("on_bevy_user_event {:?}", user_event);
    match user_event {
        UserEvent::Login(data) => tokens.add_token(data.token.token, data.user),
        UserEvent::Logout(data) => tokens.remove_token(&data.token),
        UserEvent::TokenExpired(data) => tokens.remove_token(&data.token),
        UserEvent::Register(_) => {}
    }
}
