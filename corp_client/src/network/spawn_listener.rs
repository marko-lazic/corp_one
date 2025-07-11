use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct SpawnListenerPlugin;

impl Plugin for SpawnListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_spawn_backpack);
    }
}

fn on_spawn_backpack(
    trigger: Trigger<OnAdd, Backpack>,
    r_mesh_assets: Res<MeshAssets>,
    mut commands: Commands,
) -> Result {
    let mut entity_commands = commands.get_entity(trigger.target())?;
    entity_commands
        .insert((SceneRoot(r_mesh_assets.low_poly_backpack.clone()),))
        .observe(on_use_backpack_event);
    Ok(())
}
