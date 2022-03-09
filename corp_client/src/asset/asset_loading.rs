use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;
use serde::Deserialize;

use crate::asset::paths::PATHS;
use crate::constants::state::GameState;
use crate::world::colony::colony_assets::ColonyAsset;

#[derive(AssetCollection)]
pub struct SceneAssets {
    #[asset(path = "scenes/iris/iris.scn")]
    pub iris: Handle<DynamicScene>,
    #[asset(path = "scenes/cloning/cloning.scn")]
    pub cloning: Handle<DynamicScene>,
    #[asset(path = "scenes/liberte/liberte.scn")]
    pub liberte: Handle<DynamicScene>,
}

#[derive(AssetCollection)]
pub struct ColonyAssets {
    #[asset(path = "data/colony/iris.colony")]
    pub iris: Handle<ColonyAsset>,
    #[asset(path = "data/colony/liberte.colony")]
    pub liberte: Handle<ColonyAsset>,
    #[asset(path = "data/colony/cloning.colony")]
    pub cloning: Handle<ColonyAsset>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "sound/slow-travel.wav")]
    pub slow_travel: Handle<AudioSource>,
    #[asset(path = "sound/walk.wav")]
    pub walk: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct MeshAssets {
    #[asset(path = "mesh/node/node_template.gltf#Mesh0/Primitive0")]
    pub energy_node: Handle<Mesh>,
    #[asset(path = "mesh/cube/cube.gltf#Mesh0/Primitive0")]
    pub cube: Handle<Mesh>,
    #[asset(path = "mesh/mannequiny/mannequiny-0.3.0.glb#Mesh0/Primitive0")]
    pub mannequiny: Handle<Mesh>,
    #[asset(path = "mesh/vortex_node.glb#Mesh0/Primitive0")]
    pub vortex_node: Handle<Mesh>,
}

#[derive(AssetCollection)]
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

impl AssetLoadingPlugin {
    fn print_loading_text(mut commands: Commands, asset_server: Res<AssetServer>) {
        let loading_text_bundle_entity = commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Loading",
                    TextStyle {
                        font: asset_server.load(PATHS.font_fira_sans),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            })
            .id();

        commands.insert_resource(LoadingData {
            loading_text_bundle_entity,
        });
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

    fn clean_up_loading_text(
        mut commands: Commands,
        text_query: Query<Entity, With<LoadingIndicator>>,
        loading_data: Res<LoadingData>,
    ) {
        for remove in text_query.iter() {
            commands.entity(remove).despawn_recursive();
        }
        commands
            .entity(loading_data.loading_text_bundle_entity)
            .despawn_recursive();
    }
}

impl Plugin for AssetLoadingPlugin {
    fn build(&self, mut app: &mut App) {
        AssetLoader::new(GameState::AssetLoading)
            .continue_to_state(GameState::StarMap)
            .with_collection::<MeshAssets>()
            .with_collection::<TextureAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<FontAssets>()
            .with_collection::<ColonyAssets>()
            .with_collection::<SceneAssets>()
            .build(&mut app);

        app.add_system_set(
            SystemSet::on_enter(GameState::AssetLoading)
                .with_system(Self::print_loading_text)
                .with_system(Self::start_loading),
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::AssetLoading).with_system(Self::clean_up_loading_text),
        );
    }
}

struct LoadingData {
    loading_text_bundle_entity: Entity,
}

#[derive(Component)]
struct LoadingIndicator;
