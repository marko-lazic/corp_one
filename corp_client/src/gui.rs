use bevy::app::Plugin;
use bevy::prelude::{Commands, UiCameraBundle};

pub use gui::cursor::CursorInfo;

use crate::gui::cursor::CursorPlugin;
use crate::{gui, App, MetricsPlugin};

mod cursor;
pub mod metrics;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_ui_camera);
        app.add_plugin(MetricsPlugin);
        app.add_plugin(CursorPlugin);
    }
}

impl GuiPlugin {
    fn setup_ui_camera(mut commands: Commands) {
        commands.spawn_bundle(UiCameraBundle::default());
    }
}
