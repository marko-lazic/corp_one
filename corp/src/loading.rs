use crate::paths::PATHS;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()))
        .add_system_set(
            SystemSet::on_exit(GameState::Loading).with_system(clean_up_loading.system()),
        );
    }
}

struct LoadingIndicator;

pub struct LoadingState {
    fonts: Vec<HandleUntyped>,
    audio: Vec<HandleUntyped>,
    mesh: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub fira_sans: Handle<Font>,
}

pub struct AudioAssets {
    pub slow_travel: Handle<AudioSource>,
    pub walk: Handle<AudioSource>,
}

pub struct MeshAssets {
    pub energy_node: Handle<Mesh>,
    pub cube: Handle<Mesh>,
    pub mannequiny: Handle<Mesh>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.font_fira_sans));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_slow_travel));
    audio.push(asset_server.load_untyped(PATHS.audio_walk));

    let mut mesh: Vec<HandleUntyped> = vec![];
    mesh.push(asset_server.load_untyped(PATHS.mesh_energy_node));
    mesh.push(asset_server.load_untyped(PATHS.mesh_cube));
    mesh.push(asset_server.load_untyped(PATHS.mesh_mannequiny));

    commands.insert_resource(LoadingState { fonts, audio, mesh });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }

    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        return;
    }

    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.mesh.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.font_fira_sans),
    });

    commands.insert_resource(AudioAssets {
        slow_travel: asset_server.get_handle(PATHS.audio_slow_travel),
        walk: asset_server.get_handle(PATHS.audio_walk),
    });

    commands.insert_resource(MeshAssets {
        energy_node: asset_server.get_handle(PATHS.mesh_energy_node),
        cube: asset_server.get_handle(PATHS.mesh_cube),
        mannequiny: asset_server.get_handle(PATHS.mesh_mannequiny),
    });

    state.set(GameState::Playing).unwrap();
}

fn clean_up_loading(mut commands: Commands, text_query: Query<Entity, With<LoadingIndicator>>) {
    for remove in text_query.iter() {
        commands.entity(remove).despawn_recursive();
    }
}
