use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use corp_shared::prelude::*;

pub mod prelude {
    pub use super::*;
}

#[derive(Resource)]
struct BackgroundMusicChannel;

#[derive(Resource)]
struct RunChannel;

#[derive(Resource)]
struct InteractionChannel;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<BackgroundMusicChannel>()
            .add_audio_channel::<RunChannel>()
            .add_audio_channel::<InteractionChannel>()
            .add_systems(
                OnExit(GameState::Init),
                (setup_walk_sound, setup_background_music),
            )
            .add_systems(FixedUpdate, walk_sound.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), pause_loops)
            .add_observer(play_use_sound);
    }
}

fn setup_walk_sound(walk: Res<AudioChannel<RunChannel>>, audio_assets: Res<AudioAssets>) {
    walk.play(audio_assets.running.clone())
        .looped()
        .with_volume(0.3)
        .paused();
}

fn setup_background_music(
    background: Res<AudioChannel<BackgroundMusicChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    background
        .play(audio_assets.higher_than_possible.clone())
        .looped()
        .with_volume(0.3);
}

fn pause_loops(run: Res<AudioChannel<RunChannel>>) {
    run.pause();
}

fn walk_sound(
    run: Res<AudioChannel<RunChannel>>,
    player_movement: Single<&CharacterMovement, With<Player>>,
) {
    if player_movement.is_moving() {
        run.resume().fade_in(AudioTween::default());
    } else {
        run.pause().fade_out(AudioTween::default());
    }
}

fn play_use_sound(
    _trigger: Trigger<UseEvent>,
    interaction: Res<AudioChannel<InteractionChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    interaction
        .play(audio_assets.interaction_on.clone())
        .fade_in(AudioTween::default())
        .with_volume(0.3);
}
