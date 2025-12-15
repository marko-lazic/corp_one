use crate::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct ReplicateRulesPlugin;

impl Plugin for ReplicateRulesPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Player>()
            .replicate::<Transform>() // for backpack, player not supported
            .replicate::<Backpack>()
            .replicate::<HackingTool>()
            .replicate::<Health>()
            .replicate::<CreatureName>();

        // Register client->server triggers
        app.add_client_event::<PlayerSpawnCommand>(Channel::Unordered);
        app.add_client_event::<DoorHackCommand>(Channel::Unordered);
        app.add_client_event::<LootCommand>(Channel::Unordered);
        app.add_client_event::<KillMeCommand>(Channel::Unordered);

        // Register server->client triggers
        app.add_server_event::<DoorHackedEvent>(Channel::Unordered);
        app.add_mapped_server_event::<SetupPlayerCommand>(Channel::Unordered);
        app.add_server_event::<SendDeadPlayerToCloningCommand>(Channel::Unordered);
    }
}
