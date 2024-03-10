use bevy::prelude::*;

use crate::gui::{cursor_ui::CursorPlugin, debug::DebugGuiPlugin};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugGuiPlugin, CursorPlugin));
    }
}
