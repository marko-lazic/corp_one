use bevy::app::{App, Plugin};
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_scene_hook::HookPlugin;

use crate::{
    asset::ColonyConfig,
    world::colony::{
        colony_loader::colony_loader_plugin, object_interaction::ObjectInteractionPlugin,
        vortex::VortexPlugin,
    },
};

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VortexPlugin,
            ObjectInteractionPlugin,
            HookPlugin,
            RonAssetPlugin::<ColonyConfig>::new(&["colony"]),
            DefaultPickingPlugins,
            colony_loader_plugin,
        ));
    }
}
