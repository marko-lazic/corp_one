use bevy::prelude::*;

use crate::gui::{cursor_ui::CursorPlugin, debug::DebugGuiPlugin};

mod cursor_ui;
mod debug;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugGuiPlugin, CursorPlugin));
    }
}
