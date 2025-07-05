use crate::game::ClientConnectedEvent;
use bevy::prelude::*;
use bevy_replicon::prelude::Replicated;
use corp_shared::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_client_connected);
    }
}

fn on_client_connected(trigger: Trigger<ClientConnectedEvent>, mut commands: Commands) {
    let player_e = trigger.event().0;
    commands.entity(player_e).insert((Player, Replicated));
}
