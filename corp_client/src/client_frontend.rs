use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use corp_client::prelude::*;
use corp_shared::prelude::*;

const CORP_ONE_GAME_TITLE: &str = "Corp One";
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 720;

pub struct ClientFrontendPlugin;

impl Plugin for ClientFrontendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(new_window()),
                ..default()
            }),
            GameStatePlugin,
            ShaderPlugin,
            GuiPlugin,
            bevy_framepace::FramepacePlugin,
        ));
    }
}

fn new_window() -> Window {
    Window {
        title: CORP_ONE_GAME_TITLE.to_string(),
        resolution: WindowResolution::new(WIDTH, HEIGHT),
        mode: WindowMode::Windowed,
        present_mode: PresentMode::AutoNoVsync, // Reduces input latency
        ..default()
    }
}
