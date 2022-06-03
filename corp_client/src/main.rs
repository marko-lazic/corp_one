use bevy::prelude::*;
use blender_bevy_toolkit::BlendLoadPlugin;
use iyes_loopless::prelude::AppLooplessStateExt;

use constants::state::GameState;
use constants::window;
use corp_shared::prelude::Health;
use gui::metrics::MetricsPlugin;

use crate::asset::asset_loading::AssetLoadingPlugin;
use crate::gui::GuiPlugin;
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::colony::intractable::UseEntity;
use crate::world::WorldPlugin;

mod asset;
mod constants;
mod gui;
pub mod input;
mod sound;
mod world;

#[derive(Default)]
pub struct Game {
    use_entity: UseEntity,
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
        .add_loopless_state(GameState::AssetLoading)
        .add_plugin(AssetLoadingPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(BlendLoadPlugin::default())
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
