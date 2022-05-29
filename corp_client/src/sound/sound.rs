use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

use corp_shared::prelude::*;

use crate::asset::asset_loading::AudioAssets;
use crate::constants::state::GameState;

pub struct SoundPlugin;

impl SoundPlugin {
    fn setup_live_state(audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
        audio.set_volume(0.1);
        audio.play_looped(audio_assets.walk.clone());
        audio.pause();
    }

    fn play_music(audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
        audio.set_volume(0.3);
        audio.play_looped(audio_assets.slow_travel.clone());
    }

    fn stop_audio(audio: Res<Audio>) {
        audio.stop();
    }

    fn walk_sound(audio: Res<Audio>, mut player_query: Query<&Player>) {
        if let Ok(player) = player_query.get_single_mut() {
            if player.is_moving {
                audio.resume();
            } else {
                audio.pause();
            }
        }
    }
}

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(Self::setup_live_state)
                .with_system(Self::play_music),
        );
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(Self::walk_sound));
        app.add_system_set(SystemSet::on_exit(GameState::Playing).with_system(Self::stop_audio));
    }
}
