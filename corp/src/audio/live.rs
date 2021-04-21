use crate::loading::AudioAssets;
use crate::world::agency::input::PlayerAgency;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct LivePlugin;

struct LiveChannels {
    music: AudioChannel,
    walk: AudioChannel,
}

impl Plugin for LivePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LiveChannels {
            music: AudioChannel::new("music".to_owned()),
            walk: AudioChannel::new("walk".to_owned()),
        });
        app.add_plugin(AudioPlugin);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_live_state.system())
                .with_system(play_music.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(walk_sound.system()),
        );
        app.add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_audio.system()));
    }
}

fn setup_live_state(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    channels: Res<LiveChannels>,
) {
    audio.set_volume_in_channel(0.1, &channels.walk);
    audio.play_looped_in_channel(audio_assets.walk.clone(), &channels.walk);
    audio.pause_channel(&channels.walk);
}

fn play_music(audio: Res<Audio>, audio_assets: Res<AudioAssets>, channels: Res<LiveChannels>) {
    audio.set_volume_in_channel(0.3, &channels.music);
    audio.play_looped_in_channel(audio_assets.slow_travel.clone(), &channels.music);
}

fn stop_audio(audio: Res<Audio>, channels: Res<LiveChannels>) {
    audio.stop_channel(&channels.music);
    audio.stop_channel(&channels.walk);
}

fn walk_sound(audio: Res<Audio>, channels: Res<LiveChannels>, agency: Res<PlayerAgency>) {
    if agency.moving {
        audio.resume_channel(&channels.walk);
    } else {
        audio.pause_channel(&channels.walk);
    }
}
