use crate::{prelude::*, world::colony::object_interaction::ObjectInteractionPlugin};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_scene_hook::HookPlugin;

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VortexPlugin,
            ObjectInteractionPlugin,
            HookPlugin,
            RonAssetPlugin::<ColonyConfig>::new(&["colony"]),
            colony_loader_plugin,
        ));
    }
}
