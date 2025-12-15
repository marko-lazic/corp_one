use crate::prelude::*;
use bevy::{
    prelude::*,
    remote::{
        http::{Headers, RemoteHttpPlugin},
        RemotePlugin,
    },
};
use bevy_defer::AsyncPlugin;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use bevy_skein::SkeinPlugin;

pub struct ClientBackendPlugin;

impl Plugin for ClientBackendPlugin {
    fn build(&self, app: &mut App) {
        let cors_headers = Headers::new()
            .insert("Access-Control-Allow-Origin", "https://doup.github.io")
            .insert("Access-Control-Allow-Headers", "Content-Type");

        app.add_plugins((
            SkeinPlugin { handle_brp: false },
            RemotePlugin::default(),
            RemoteHttpPlugin::default().with_headers(cors_headers),
            AssetLoadingPlugin,
            EntropyPlugin::<WyRand>::default(),
            AsyncPlugin::default_settings(),
            SoundPlugin,
            LoadColonyPlugin,
            WorldPlugin,
            ClientNetPlugin,
        ));
    }
}
