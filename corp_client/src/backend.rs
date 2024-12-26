use crate::prelude::*;
use bevy::prelude::*;

pub struct BackendPlugin;

impl Plugin for BackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameStatePlugin,
            AssetLoadingPlugin,
            SoundPlugin,
            WorldPlugin,
            ClientNetPlugin,
        ));
    }
}
