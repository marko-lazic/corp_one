use bevy::prelude::*;
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(init_clients)
            .add_observer(on_add_connected_client)
            .add_observer(on_remove_connected_client);
    }
}

fn init_clients(
    player_spawn: On<FromClient<PlayerSpawnCommand>>,
    mut commands: Commands,
) -> Result {
    info!(
        "Received PlayerSpawnCommand from {:?}",
        player_spawn.client_id
    );

    let client_entity = player_spawn
        .client_id
        .entity()
        .expect("Client ID is not an entity ID");

    // Create player
    commands.entity(client_entity).insert((
        Player,
        Replicated,
        Signature::of::<Player>(),
        Health::default(),
        // Health::from(response.hit_points),
        // CreatureName(response.character_name),
        // AuthorizedClient,
    ));

    // Inform the client that he is ready
    commands.server_trigger(ToClients {
        targets: SendTargets::Single(player_spawn.client_id),
        message: SetupPlayerCommand {
            player_target: player_spawn
                .client_id
                .entity()
                .expect("Client ID is not an entity ID"),
        },
    });
    Ok(())
}

fn on_add_connected_client(add: On<Add<ConnectedClient>>) {
    info!("OnAdd ClientConnected {:?}", add.entity);
    // We don't do here anything at the moment because the client asks to get spawned
}

fn on_remove_connected_client(remove: On<Remove<ConnectedClient>>) {
    info!("OnRemove ClientConnected {:?}", remove.entity);
}
