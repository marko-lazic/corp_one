use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};

use crate::asset::asset_loading::AudioAssets;
use crate::state::GameState;
use crate::world::player::Player;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin);
        app.add_systems(
            (Self::setup_live_state, Self::play_music)
                .chain()
                .in_set(OnUpdate(GameState::Playing)),
        );
        app.add_system(Self::walk_sound.in_set(OnUpdate(GameState::Playing)));
        app.add_system(Self::stop_audio.in_schedule(OnExit(GameState::Playing)));
    }
}

impl SoundPlugin {
    fn setup_live_state(audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
        audio
            .play(audio_assets.walk.clone())
            .looped()
            .with_volume(0.1);
        audio.pause();
    }

    fn play_music(audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
        audio
            .play(audio_assets.slow_travel.clone())
            .looped()
            .with_volume(0.3);
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
