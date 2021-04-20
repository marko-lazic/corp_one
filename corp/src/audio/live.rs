use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use std::collections::HashMap;

pub struct LivePlugin;

struct AudioState {
    audio_loaded: bool,
    channels: HashMap<AudioChannel, ChannelAudioState>,
    music_channel: AudioChannel,
    slow_travel: Handle<AudioSource>,
}

struct ChannelAudioState {
    stopped: bool,
    paused: bool,
    loop_started: bool,
    volume: f32,
}

impl Default for ChannelAudioState {
    fn default() -> Self {
        ChannelAudioState {
            volume: 1.0,
            stopped: true,
            loop_started: false,
            paused: false,
        }
    }
}

impl Plugin for LivePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin);
        app.add_startup_system(prepare_audio.system());
        app.add_system(check_audio_loading.system());
        app.add_system(start_background_music_loop.system());
    }
}

fn check_audio_loading(mut audio_state: ResMut<AudioState>, asset_server: ResMut<AssetServer>) {
    if audio_state.audio_loaded
        || LoadState::Loaded != asset_server.get_load_state(&audio_state.slow_travel)
    {
        return;
    }
    audio_state.audio_loaded = true;
}

fn prepare_audio(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let mut channels = HashMap::new();
    let music_channel = AudioChannel::new("music".to_owned());
    channels.insert(music_channel.clone(), ChannelAudioState::default());

    let slow_travel = asset_server.load("sounds/slow-travel.wav");

    let audio_state = AudioState {
        audio_loaded: false,
        channels,
        music_channel,
        slow_travel,
    };

    commands.insert_resource(audio_state);
}

fn start_background_music_loop(audio: Res<Audio>, mut audio_state: ResMut<AudioState>) {
    if !audio_state.audio_loaded {
        return;
    }

    let mut channel_audio_state = audio_state
        .channels
        .get_mut(&AudioChannel::new("music".to_owned()))
        .unwrap();

    if channel_audio_state.loop_started {
        return;
    }

    channel_audio_state.loop_started = true;
    channel_audio_state.stopped = false;
    audio.play_looped_in_channel(audio_state.slow_travel.clone(), &audio_state.music_channel);
}
