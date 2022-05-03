use bevy::app::Plugin;

pub use gui::cursor::CursorInfo;

use crate::gui::cursor::CursorPlugin;
use crate::{gui, App, MetricsPlugin};

mod cursor;
pub mod metrics;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MetricsPlugin);
        app.add_plugin(CursorPlugin);
    }
}
