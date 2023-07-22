use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
};
use bevy_kira_audio::AudioSource;
use serde::Deserialize;

use crate::{
    asset::PATHS,
    state::{Despawn, GameState},
    world::colony::colony_assets::ColonyAsset,
};

/// Exported using Blender export glTF 2.0 with settings enabled
///
/// Punctual Lights
///
/// Data/Lighting Lighting Mode Unitless
#[derive(Resource, AssetCollection)]
pub struct SceneAssets {
    #[asset(path = "scenes/iris/iris.glb#Scene0")]
    pub iris: Handle<Scene>,
    #[asset(path = "scenes/cloning/cloning.glb#Scene0")]
    pub cloning: Handle<Scene>,
    #[asset(path = "scenes/liberte/liberte.glb#Scene0")]
    pub liberte: Handle<Scene>,
}

#[derive(Resource, AssetCollection)]
pub struct ColonyAssets {
    #[asset(path = "data/colony/iris.colony")]
    pub iris: Handle<ColonyAsset>,
    #[asset(path = "data/colony/liberte.colony")]
    pub liberte: Handle<ColonyAsset>,
    #[asset(path = "data/colony/cloning.colony")]
    pub cloning: Handle<ColonyAsset>,
}

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(Resource, AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "sound/slow-travel.wav")]
    pub slow_travel: Handle<AudioSource>,
    #[asset(path = "sound/walk.wav")]
    pub walk: Handle<AudioSource>,
}

#[derive(Resource, AssetCollection)]
pub struct MeshAssets {
    #[asset(path = "mesh/node/node_template.gltf#Mesh0/Primitive0")]
    pub energy_node: Handle<Mesh>,
    #[asset(path = "mesh/cube/cube.gltf#Mesh0/Primitive0")]
    pub cube: Handle<Mesh>,
    #[asset(path = "mesh/vortex_node.glb#Mesh0/Primitive0")]
    pub vortex_node: Handle<Mesh>,
}

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
    pub cube: Handle<StandardMaterial>,
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
            LoadingState::new(GameState::Loading).continue_to_state(GameState::StarMap),
        )
        .add_collection_to_loading_state::<_, PlayerAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, MeshAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, ColonyAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, SceneAssets>(GameState::Loading)
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
                    font: asset_server.load(PATHS.font_fira_sans),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            ..Default::default()
        },
        Despawn,
    ));
}

fn start_loading(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(MaterialAssets {
        cube: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..Default::default()
        }),
        green_material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        blue_material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
        sky_blue_material: materials.add(Color::rgb(0.55, 0.71, 0.73).into()),
        pink_material: materials.add(Color::PINK.into()),
        orange_red_material: materials.add(Color::ORANGE_RED.into()),
        sea_green_material: materials.add(Color::SEA_GREEN.into()),
    });
}
