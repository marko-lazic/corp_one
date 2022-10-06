use bevy::app::Plugin;

pub use gui::cursor::CursorInfo;

use crate::{App, gui, MetricsPlugin};
use crate::gui::cursor::CursorPlugin;

mod cursor;
pub mod metrics;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MetricsPlugin);
        app.add_plugin(CursorPlugin);
    }
}
