use crate::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct ReplicateRulesPlugin;

impl Plugin for ReplicateRulesPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Player>()
            .replicate::<Transform>()
            .replicate::<Backpack>()
            .replicate::<HackingTool>()
            .replicate::<Health>()
            .replicate::<CreatureName>();

        // Register client->server triggers
        app.add_client_trigger::<PlayerSpawnClientCommand>(Channel::Unordered);

        app.add_client_trigger::<DoorHackCommand>(Channel::Unordered);
        app.add_client_trigger::<LootCommand>(Channel::Unordered);
        app.add_client_trigger::<KillMeCommand>(Channel::Unordered);

        // Register server->client triggers
        app.add_server_trigger::<DoorHackedEvent>(Channel::Unordered);
        app.add_server_trigger::<SetupPlayerServerCommand>(Channel::Unordered);
        app.add_server_trigger::<SendDeadPlayerToCloningCommand>(Channel::Unordered);
    }
}
