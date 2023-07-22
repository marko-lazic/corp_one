use bevy::app::Plugin;

pub use gui::cursor_ui::CursorVisibility;

use crate::gui::cursor_ui::CursorPlugin;
use crate::{gui, App, MetricsPlugin};

mod cursor_ui;
pub mod metrics;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MetricsPlugin, CursorPlugin));
    }
}
