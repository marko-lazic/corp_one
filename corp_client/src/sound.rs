use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};

use corp_shared::prelude::*;

use crate::{asset::AudioAssets, state::GameState, world::prelude::CharacterMovement};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(
                Update,
                (setup_live_state, play_music)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, walk_sound.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), stop_audio);
    }
}

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

fn walk_sound(audio: Res<Audio>, mut player_query: Query<&CharacterMovement, With<Player>>) {
    if let Ok(player) = player_query.get_single_mut() {
        if player.is_moving() {
            audio.resume();
        } else {
            audio.pause();
        }
    }
}
