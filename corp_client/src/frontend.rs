use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use corp_client::prelude::*;

const CORP_ONE_GAME_TITLE: &str = "Corp One";
const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 720.0;

pub struct FrontendPlugin;

impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(new_window()),
                ..default()
            }),
            ShaderPlugin,
            GuiPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
            bevy_framepace::FramepacePlugin,
            FrontendReplicationPlugin,
        ));
    }
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
