use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use corp_shared::prelude::*;

/// Exported using Blender export glTF 2.0 with settings enabled
///
/// Include->Punctual Lights
///
/// Data/Lighting->Lighting Mode Unitless
#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    /// Iris Relaxed Alien Adventure
    #[asset(path = "scenes/iris/iris.glb#Scene0")]
    pub iris: Handle<Scene>,
    /// Cloning Facility
    #[asset(path = "scenes/cloning/cloning.glb#Scene0")]
    pub cloning: Handle<Scene>,
    /// Liberte stronghold of The Democratic
    #[asset(path = "scenes/liberte/liberte.glb#Scene0")]
    pub liberte: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Anonymous Pro.ttf")]
    pub default_font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sound/andrewkn-higher-than-possible.ogg")]
    pub higher_than_possible: Handle<AudioSource>,
    #[asset(path = "sound/running.wav")]
    pub running: Handle<AudioSource>,
    #[asset(path = "sound/future-sounds-4.wav")]
    pub interaction_on: Handle<AudioSource>,
    #[asset(path = "sound/future-sounds-9.wav")]
    pub _interaction_off: Handle<AudioSource>,
}

#[derive(Resource, AssetCollection)]
pub struct MeshAssets {
    #[asset(path = "mesh/backpack.glb#Scene0")]
    pub low_poly_backpack: Handle<Scene>,
}

#[derive(Resource, AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "mesh/mannequiny.gltf#Scene0")]
    pub mannequiny: Handle<Scene>,
    #[asset(path = "mesh/mannequiny.gltf#Animation9")]
    pub run: Handle<AnimationClip>,
    #[asset(path = "mesh/mannequiny.gltf#Animation7")]
    pub idle: Handle<AnimationClip>,
}

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "starmap/nebula.png")]
    pub nebula: Handle<Image>,
}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Init)
                .load_collection::<PlayerAssets>()
                .load_collection::<MeshAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<FontAssets>()
                .load_collection::<SceneAssets>()
                .continue_to_state(GameState::Load(Colony::StarMap)),
        );
    }
}
