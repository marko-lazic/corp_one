use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use corp_shared::prelude::*;

pub mod prelude {
    pub use super::*;
}

#[derive(Resource)]
struct BackgroundMusic;

#[derive(Resource)]
struct RunSound;

#[derive(Resource)]
struct InteractionSound;

#[derive(Event)]
pub struct InteractionSoundEvent;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_event::<InteractionSoundEvent>()
            .add_audio_channel::<BackgroundMusic>()
            .add_audio_channel::<RunSound>()
            .add_audio_channel::<InteractionSound>()
            .add_systems(
                OnExit(GameState::Init),
                (setup_walk_sound, setup_background_music),
            )
            .add_systems(
                FixedUpdate,
                (walk_sound, play_interaction_event).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), pause_loops);
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

fn pause_loops(run: Res<AudioChannel<RunSound>>) {
    run.pause();
}

fn walk_sound(
    run: Res<AudioChannel<RunSound>>,
    player_movement: Single<&CharacterMovement, With<Player>>,
) {
    if player_movement.is_moving() {
        run.resume().fade_in(AudioTween::default());
    } else {
        run.pause().fade_out(AudioTween::default());
    }
}

fn play_interaction_event(
    interaction: Res<AudioChannel<InteractionSound>>,
    audio_assets: Res<AudioAssets>,
    mut ev_interaction_sound: EventReader<InteractionSoundEvent>,
) {
    for _ev in &mut ev_interaction_sound.read() {
        interaction
            .play(audio_assets.interaction_on.clone())
            .fade_in(AudioTween::default())
            .with_volume(0.3);
    }
}
