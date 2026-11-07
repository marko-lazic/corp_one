use bevy::prelude::*;
use corp_types::auth::{User, UserEvent};
use std::collections::HashSet;

#[derive(Event)]
pub struct BevyUserEvent(pub UserEvent);

/// Authentication metadata belongs to each session, rather than a unique world resource.
#[derive(Component)]
pub struct SessionAuthToken(pub String);

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

fn on_bevy_user_event(bevy_user_event: On<BevyUserEvent>, mut tokens: ResMut<Tokens>) {
    let user_event = bevy_user_event.event().0.clone();
    match user_event {
        UserEvent::Login(data) => tokens.add_token(data.token.token, data.user),
        UserEvent::Logout(data) => tokens.remove_token(&data.token),
        UserEvent::TokenExpired(data) => tokens.remove_token(&data.token),
        UserEvent::Register(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeronet::io::{
        AeronetIoPlugin,
        connection::{DisconnectReason, Disconnected},
    };
    use bevy::ecs::resource::IsResource;
    use corp_shared::prelude::AuthToken;

    #[test]
    fn session_tokens_are_independent_and_disconnect_preserves_resources() {
        let mut app = App::new();
        app.add_plugins(AeronetIoPlugin)
            .insert_resource(AuthToken("global-token".into()));

        let first = app.world_mut().spawn(SessionAuthToken("first".into())).id();
        let second = app
            .world_mut()
            .spawn(SessionAuthToken("second".into()))
            .id();
        assert!(app.world().get::<IsResource>(first).is_none());
        assert!(app.world().get::<IsResource>(second).is_none());

        app.world_mut().trigger(Disconnected {
            entity: first,
            reason: DisconnectReason::by_peer("client disconnected"),
        });
        app.world_mut().flush();

        assert!(app.world().get_entity(first).is_err());
        assert_eq!(
            app.world().get::<SessionAuthToken>(second).unwrap().0,
            "second"
        );
        assert_eq!(app.world().resource::<AuthToken>().0, "global-token");
    }
}
