use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

use corp_shared::prelude::*;

use crate::asset::asset_loading::AudioAssets;
use crate::constants::state::GameState;
use crate::constants::tick;

pub struct SoundPlugin;

impl SoundPlugin {
    fn setup_live_state(
        audio: Res<Audio>,
        audio_assets: Res<AudioAssets>,
        channels: Res<SoundChannels>,
    ) {
        audio.set_volume_in_channel(0.1, &channels.walk);
        audio.play_looped_in_channel(audio_assets.walk.clone(), &channels.walk);
        audio.pause_channel(&channels.walk);
    }

    fn play_music(audio: Res<Audio>, audio_assets: Res<AudioAssets>, channels: Res<SoundChannels>) {
        audio.set_volume_in_channel(0.3, &channels.music);
        audio.play_looped_in_channel(audio_assets.slow_travel.clone(), &channels.music);
    }

    fn stop_audio(audio: Res<Audio>, channels: Res<SoundChannels>) {
        audio.stop_channel(&channels.music);
        audio.stop_channel(&channels.walk);
    }

    fn walk_sound(
        audio: Res<Audio>,
        channels: Res<SoundChannels>,
        mut player_query: Query<&Player>,
    ) {
        if let Ok(player) = player_query.single_mut() {
            if player.is_moving {
                audio.resume_channel(&channels.walk);
            } else {
                audio.pause_channel(&channels.walk);
            }
        }
    }
}

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(SoundChannels {
            music: AudioChannel::new("music".to_owned()),
            walk: AudioChannel::new("walk".to_owned()),
        });
        app.add_plugin(AudioPlugin);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(Self::setup_live_state.system())
                .with_system(Self::play_music.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::walk_sound.system()),
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(Self::stop_audio.system()),
        );
    }
}

struct SoundChannels {
    music: AudioChannel,
    walk: AudioChannel,
}