use bevy::prelude::*;

use corp_shared::prelude::*;

use crate::world::colony::barrier::BarrierPlugin;

pub struct ObjectInteractionPlugin;

impl Plugin for ObjectInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackpackInteractionEvent>() // Does not have plugin ATM
            .add_plugins(BarrierPlugin);
    }
}
