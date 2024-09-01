use crate::world::colony::barrier::BarrierPlugin;
use bevy::prelude::*;
use corp_shared::prelude::despawn_empty_backpack_system;

pub struct ObjectInteractionPlugin;

impl Plugin for ObjectInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BarrierPlugin)
            .add_systems(FixedUpdate, despawn_empty_backpack_system);
    }
}
