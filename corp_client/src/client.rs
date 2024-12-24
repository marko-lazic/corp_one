use crate::prelude::*;
use bevy::prelude::*;

pub struct CorpClientPlugin;

impl Plugin for CorpClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameStatePlugin,
            AssetLoadingPlugin,
            SoundPlugin,
            WorldPlugin,
        ));
    }
}
