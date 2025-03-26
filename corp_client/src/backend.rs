use crate::prelude::*;
use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

pub struct BackendPlugin;

impl Plugin for BackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetLoadingPlugin,
            EntropyPlugin::<WyRand>::default(),
            SoundPlugin,
            WorldPlugin,
            ClientNetPlugin,
        ));
    }
}
