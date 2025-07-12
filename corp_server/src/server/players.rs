use bevy::prelude::*;
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(init_clients)
            .add_observer(connect_clients)
            .add_observer(despawn_clients);
    }
}

fn init_clients(
    trigger: Trigger<FromClient<PlayerSpawnClientCommand>>,
    mut commands: Commands,
) -> Result {
    info!(
        "Received client player spawn command {:?}",
        trigger.client_entity
    );

    let client_entity = trigger.client_entity;

    // Create player
    commands.entity(client_entity).insert((
        Player,
        Replicated,
        Transform::default(),
        Health::default(),
        // Health::from(response.hit_points),
        // CreatureName(response.character_name),
        AuthorizedClient,
        ClientEntityMap::default(),
    ));

    // Inform the client that he is ready
    commands.server_trigger_targets(
        ToClients {
            mode: SendMode::Direct(client_entity),
            event: SetupPlayerServerCommand,
        },
        client_entity,
    );
    Ok(())
}

fn connect_clients(trigger: Trigger<OnAdd, ConnectedClient>) {
    info!("OnAdd ClientConnected {:?}", trigger.target());
    // We don't do here anything at the moment because the client asks to get spawned
}

fn despawn_clients(trigger: Trigger<OnRemove, ConnectedClient>, mut commands: Commands) {
    info!("OnRemove ClientConnected {:?}", trigger.target());
    commands.entity(trigger.target()).try_despawn();
}
