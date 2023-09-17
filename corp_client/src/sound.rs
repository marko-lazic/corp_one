use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, AudioChannel};

use corp_shared::prelude::*;

use crate::{asset::AudioAssets, state::GameState, world::prelude::CharacterMovement};

#[derive(Resource)]
struct BackgroundMusic;

#[derive(Resource)]
struct RunSound;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<BackgroundMusic>()
            .add_audio_channel::<RunSound>()
            .add_systems(
                OnEnter(GameState::Playing),
                (setup_walk_sound, setup_background_music),
            )
            .add_systems(Update, walk_sound.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), stop_audio);
    }
}

fn setup_walk_sound(walk: Res<AudioChannel<RunSound>>, audio_assets: Res<AudioAssets>) {
    walk.play(audio_assets.running.clone())
        .looped()
        .with_volume(0.3)
        .paused();
}

fn setup_background_music(
    background: Res<AudioChannel<BackgroundMusic>>,
    audio_assets: Res<AudioAssets>,
) {
    background
        .play(audio_assets.higher_than_possible.clone())
        .looped()
        .with_volume(0.3);
}

fn stop_audio(audio: Res<Audio>) {
    audio.stop();
}

fn walk_sound(
    run: Res<AudioChannel<RunSound>>,
    mut player_query: Query<&CharacterMovement, With<Player>>,
) {
    if let Ok(player) = player_query.get_single_mut() {
        if player.is_moving() {
            run.resume().fade_in(AudioTween::default());
        } else {
            run.pause().fade_out(AudioTween::default());
        }
    }
}
