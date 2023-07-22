use std::time::Duration;

use bevy::{
    asset::ChangeWatcher,
    input::common_conditions::input_toggle_active,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use corp_shared::prelude::Health;

use crate::{
    asset::{AssetLoadingPlugin, ColonyConfig},
    gui::GuiPlugin,
    state::GameStatePlugin,
    world::WorldPlugin,
};

mod asset;
mod gui;
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
    current_colony_config: Handle<ColonyConfig>,
    health: Health,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(Msaa::Sample4)
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(new_window()),
                    ..default()
                }),
        )
        .add_plugins((
            GameStatePlugin,
            AssetLoadingPlugin,
            GuiPlugin,
            WorldPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        ))
        .run();
}

fn new_window() -> Window {
    Window {
        title: CORP_ONE_GAME_TITLE.to_string(),
        resolution: (WIDTH, HEIGHT).into(),
        mode: WindowMode::BorderlessFullscreen,
        present_mode: PresentMode::AutoNoVsync, // Reduces input latency
        ..default()
    }
}
