use bevy::{color::palettes::tailwind, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use serde::Deserialize;

use crate::{
    asset::prelude::{ColonyConfig, PATH},
    state::{Despawn, GameState},
};

/// Exported using Blender export glTF 2.0 with settings enabled
///
/// Include->Punctual Lights
///
/// Data/Lighting->Lighting Mode Unitless
#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "scenes/iris/iris.glb#Scene0")]
    pub iris: Handle<Scene>,
    #[asset(path = "scenes/cloning/cloning.glb#Scene0")]
    pub cloning: Handle<Scene>,
    #[asset(path = "scenes/liberte/liberte.glb#Scene0")]
    pub liberte: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct ColonyConfigAssets {
    #[asset(path = "data/colony/iris.colony")]
    pub iris: Handle<ColonyConfig>,
    #[asset(path = "data/colony/liberte.colony")]
    pub liberte: Handle<ColonyConfig>,
    #[asset(path = "data/colony/cloning.colony")]
    pub cloning: Handle<ColonyConfig>,
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
pub struct MeshAssets {}

#[derive(Resource, AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "mesh/mannequiny/mannequiny.gltf#Scene0")]
    pub mannequiny: Handle<Scene>,
    #[asset(path = "mesh/mannequiny/mannequiny.gltf#Animation9")]
    pub run: Handle<AnimationClip>,
    #[asset(path = "mesh/mannequiny/mannequiny.gltf#Animation7")]
    pub idle: Handle<AnimationClip>,
}

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "starmap/nebula.png")]
    pub nebula: Handle<Image>,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum MaterialAsset {
    Green,
    Blue,
    SkyBlue,
    OrangeRed,
    SeaGreen,
    Unknown,
}

impl Default for MaterialAsset {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Resource)]
pub struct MaterialAssets {
    pub green_material: Handle<StandardMaterial>,
    pub blue_material: Handle<StandardMaterial>,
    pub sky_blue_material: Handle<StandardMaterial>,
    pub pink_material: Handle<StandardMaterial>,
    pub orange_red_material: Handle<StandardMaterial>,
    pub sea_green_material: Handle<StandardMaterial>,
}

impl MaterialAssets {
    pub fn get_material(&self, asset_material: &MaterialAsset) -> Handle<StandardMaterial> {
        match asset_material {
            MaterialAsset::Green => self.green_material.clone(),
            MaterialAsset::Blue => self.blue_material.clone(),
            MaterialAsset::SkyBlue => self.sky_blue_material.clone(),
            MaterialAsset::OrangeRed => self.orange_red_material.clone(),
            MaterialAsset::SeaGreen => self.sea_green_material.clone(),
            MaterialAsset::Unknown => self.pink_material.clone(),
        }
    }
}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::StarMap)
                .load_collection::<PlayerAssets>()
                .load_collection::<MeshAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<FontAssets>()
                .load_collection::<ColonyConfigAssets>()
                .load_collection::<SceneAssets>(),
        )
        .add_systems(OnEnter(GameState::Loading), (setup, start_loading).chain());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), Despawn));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Loading",
                TextStyle {
                    font: asset_server.load(PATH.default_font),
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            ..Default::default()
        },
        Despawn,
    ));
}

fn start_loading(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(MaterialAssets {
        green_material: materials.add(Color::srgb(0.1, 0.2, 0.1)),
        blue_material: materials.add(Color::srgb(0.1, 0.4, 0.8)),
        sky_blue_material: materials.add(Color::srgb(0.55, 0.71, 0.73)),
        pink_material: materials.add(Color::from(tailwind::PINK_700)),
        orange_red_material: materials.add(Color::from(tailwind::ORANGE_700)),
        sea_green_material: materials.add(Color::from(tailwind::GREEN_700)),
    });
}
