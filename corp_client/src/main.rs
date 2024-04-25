use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    asset::AssetLoadingPlugin, gui::gui::GuiPlugin, sound::SoundPlugin, state::GameStatePlugin,
    world::prelude::WorldPlugin,
};

mod asset;
mod gui;
pub mod sound;
pub mod state;
pub mod util;
mod world;

const CORP_ONE_GAME_TITLE: &str = "Corp One";
const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(new_window()),
            ..default()
        }))
        .add_plugins((
            GameStatePlugin,
            AssetLoadingPlugin,
            SoundPlugin,
            GuiPlugin,
            WorldPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
            bevy_framepace::FramepacePlugin,
        ))
        .run();
}

fn new_window() -> Window {
    Window {
        title: CORP_ONE_GAME_TITLE.to_string(),
        resolution: (WIDTH, HEIGHT).into(),
        mode: WindowMode::Windowed,
        present_mode: PresentMode::AutoNoVsync, // Reduces input latency
        ..default()
    }
}
