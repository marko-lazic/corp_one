use crate::prelude::*;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use corp_shared::world::structure::StructurePlugin;

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VortexPlugin,
            BarrierPlugin,
            StructurePlugin,
            RonAssetPlugin::<ColonyConfig>::new(&["colony"]),
            colony_loader_plugin,
        ));
    }
}
