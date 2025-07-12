use corp_types::prelude::*;
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
    Actor,
};
use std::collections::HashSet;

#[derive(Debug)]
pub struct IsTokenValid(pub String);

impl Message<IsTokenValid> for Tokens {
    type Reply = bool;

    fn handle(
        &mut self,
        msg: IsTokenValid,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> impl Future<Output = Self::Reply> + Send {
        async move { self.is_valid(&msg.0) }
    }
}

#[derive(Default, Debug)]
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

impl Actor for Tokens {
    type Args = Self;
    type Error = ();

    fn on_start(
        args: Self::Args,
        _actor_ref: ActorRef<Self>,
    ) -> impl Future<Output = std::result::Result<Self, Self::Error>> + Send {
        async move { Ok(args) }
    }
}

impl Message<AuthenticationEvent> for Tokens {
    type Reply = ();

    fn handle(
        &mut self,
        msg: AuthenticationEvent,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> impl Future<Output = Self::Reply> + Send {
        async move {
            match msg.event_type {
                AuthEventType::Login(data) => {
                    self.add_token(data.token.token, data.user.clone());
                    println!("Token added for user: {:?}", data.user);
                }
                AuthEventType::Logout(data) => {
                    self.remove_token(&data.token);
                    println!("Token removed: {}", data.token);
                }
                AuthEventType::TokenExpired(data) => {
                    self.remove_token(&data.token);
                    println!("Token expired: {}", data.token);
                }
            }
        }
    }
}
