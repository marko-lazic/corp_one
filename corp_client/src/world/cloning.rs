use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct CloningLocalPlugin;

impl Plugin for CloningLocalPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_send_dead_player_to_cloning);
    }
}

fn on_send_dead_player_to_cloning(
    _send_dead_player_to_cloning_command: On<SendDeadPlayerToCloningCommand>,
    mut commands: Commands,
) {
    info!("Received SendDeadPlayerToCloningCommand. Server should control this!");
    commands.trigger(RequestConnect(Colony::Cloning));
}
