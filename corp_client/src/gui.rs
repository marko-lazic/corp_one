use bevy::prelude::*;

use crate::gui::cursor_ui::CursorPlugin;
use crate::gui::debug::DebugGuiPlugin;

mod cursor_ui;
mod debug;

pub use super::gui::cursor_ui::CursorVisibility;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugGuiPlugin, CursorPlugin));
    }
}
