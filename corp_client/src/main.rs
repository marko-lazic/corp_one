use bevy::prelude::*;

use constants::state::GameState;
use constants::window;
use gui::metrics::MetricsPlugin;

use crate::asset::asset_loading::AssetLoadingPlugin;
use crate::world::WorldPlugin;

mod asset;
mod connection;
mod constants;
mod gui;
pub mod input;
mod sound;
mod world;

#[derive(Default)]
pub struct Game {
    player_entity: Option<Entity>,
    camera_transform: Option<Transform>,
    camera_center: Vec3,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(create_window_descriptor())
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_plugin(AssetLoadingPlugin)
        .add_state(GameState::AssetLoading)
        .add_plugin(MetricsPlugin)
        .add_plugin(WorldPlugin)
        // .add_plugin(ConnectionPlugin)
        .run();
}

fn create_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: window::CORP_ONE_GAME_TITLE.to_string(),
        width: window::WIDTH,
        height: window::HEIGHT,
        ..Default::default()
    }
}
