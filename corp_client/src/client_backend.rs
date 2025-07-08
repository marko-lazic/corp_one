use crate::prelude::*;
use bevy::prelude::*;
use bevy_defer::AsyncPlugin;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

pub struct ClientBackendPlugin;

impl Plugin for ClientBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetLoadingPlugin,
            EntropyPlugin::<WyRand>::default(),
            AsyncPlugin::default_settings(),
            SoundPlugin,
            WorldPlugin,
            ClientNetPlugin,
        ));
    }
}
