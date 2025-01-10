use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct FrontendReplicationPlugin;

impl Plugin for FrontendReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            receive_backpack_insert.run_if(in_state(GameState::Playing)),
        );
    }
}
fn receive_backpack_insert(
    r_mesh_assets: Res<MeshAssets>,
    mut reader: EventReader<ComponentInsertEvent<Backpack>>,
    mut commands: Commands,
) {
    for event in reader.read() {
        if let Some(mut entity_commands) = commands.get_entity(event.entity()) {
            entity_commands
                .insert((SceneRoot(r_mesh_assets.low_poly_backpack.clone()),))
                .observe(on_use_backpack_event)
                .observe(on_use_backpack_action_event);
        } else {
            warn!(
                "Backpack with entity {:?}  does not exist yet.",
                event.entity()
            );
        };
    }
}
