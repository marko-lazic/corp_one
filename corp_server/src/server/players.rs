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

fn init_clients(trigger: Trigger<FromClient<ClientPlayerSpawnCommand>>, mut commands: Commands) {
    info!(
        "Received client player spawn command {:?}",
        trigger.client_entity
    );
    // Skip authorization for now

    // Create player
    commands.entity(trigger.client_entity).insert((
        Player,
        AuthorizedClient,
        ClientEntityMap::default(),
    ));

    // Inform the client that he is ready
    commands.server_trigger_targets(
        ToClients {
            mode: SendMode::Direct(trigger.client_entity),
            event: MakeLocal,
        },
        trigger.client_entity,
    );
}

fn connect_clients(trigger: Trigger<OnAdd, ConnectedClient>) {
    info!("OnAdd ClientConnected {:?}", trigger.target());
}

fn despawn_clients(trigger: Trigger<OnRemove, ConnectedClient>) {
    info!("OnRemove ClientConnected {:?}", trigger.target());
}
