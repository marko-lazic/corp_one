use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

use crate::asset::paths::PATHS;
use crate::constants::state::GameState;

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
}

pub struct MaterialAssets {
    pub cube: Handle<StandardMaterial>,
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
    fn build(&self, mut app: &mut AppBuilder) {
        AssetLoader::new(GameState::AssetLoading, GameState::Playing)
            .with_collection::<MeshAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<FontAssets>()
            .build(&mut app);

        app.add_system_set(
            SystemSet::on_enter(GameState::AssetLoading)
                .with_system(Self::print_loading_text.system())
                .with_system(Self::start_loading.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::AssetLoading)
                .with_system(Self::clean_up_loading_text.system()),
        );
    }
}

struct LoadingData {
    loading_text_bundle_entity: Entity,
}

struct LoadingIndicator;
