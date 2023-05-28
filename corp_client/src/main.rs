use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use corp_shared::prelude::Health;
use gui::metrics::MetricsPlugin;

use crate::asset::asset_loading::AssetLoadingPlugin;
use crate::gui::GuiPlugin;
use crate::state::GameStatePlugin;
use crate::world::colony::colony_assets::ColonyAsset;
use crate::world::WorldPlugin;

mod asset;
mod gui;
pub mod input;
mod sound;
pub mod state;
pub mod util;
mod world;

const CORP_ONE_GAME_TITLE: &str = "Corp One";
const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 720.0;

#[derive(Resource, Default)]
pub struct Game {
    use_entity: Option<Entity>,
    cursor_locked: bool,
    player_entity: Option<Entity>,
    camera_transform: Option<Transform>,
    camera_center: Vec3,
    current_colony_asset: Handle<ColonyAsset>,
    health: Health,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(Msaa::Sample4)
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(new_window()),
                    ..default()
                }),
        )
        .add_plugin(GameStatePlugin)
        .add_plugin(AssetLoadingPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .run();
}

fn new_window() -> Window {
    Window {
        title: CORP_ONE_GAME_TITLE.to_string(),
        resolution: (WIDTH, HEIGHT).into(),
        present_mode: PresentMode::AutoNoVsync, // Reduces input latency
        ..Default::default()
    }
}
