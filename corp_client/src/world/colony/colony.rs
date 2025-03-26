use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::{prelude::action::ActionPlugin, world::structure::StructurePlugin};

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VortexPlugin,
            BarrierPlugin,
            StructurePlugin,
            AreaPlugin,
            ActionPlugin,
            colony_loader_plugin,
        ));
    }
}
