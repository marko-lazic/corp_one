use bevy::prelude::*;
use blender_bevy_toolkit::BlendLoadPlugin;

use constants::state::GameState;
use constants::window;
use corp_shared::prelude::Health;
use gui::metrics::MetricsPlugin;

use crate::asset::asset_loading::AssetLoadingPlugin;
use crate::gui::GuiPlugin;
use crate::world::colony::colony_assets::ColonyAsset;
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
    cursor_locked: bool,
    player_entity: Option<Entity>,
    camera_transform: Option<Transform>,
    camera_center: Vec3,
    current_colony_asset: Handle<ColonyAsset>,
    health: Health,
    scene_handle: Handle<DynamicScene>,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(create_window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadingPlugin)
        .add_state(GameState::AssetLoading)
        .add_plugin(GuiPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(BlendLoadPlugin::default())
        .add_plugin(bevy_framepace::FramepacePlugin {
            enabled: true,
            framerate_limit: bevy_framepace::FramerateLimit::Auto,
            warn_on_frame_drop: false,
            safety_margin: std::time::Duration::from_millis(2),
        })
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
