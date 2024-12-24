use crate::{gui::cursor_ui::CursorPlugin, prelude::*};
use bevy::prelude::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugGuiPlugin, CursorPlugin))
            .add_systems(OnEnter(GameState::Loading), setup.chain());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, StateScoped(GameState::Loading)));
    commands.spawn((
        Text::new("Loading"),
        TextFont::from_font(asset_server.load(ASSET_PATH.default_font)).with_font_size(40.0),
        TextColor::from(Color::srgb(0.9, 0.9, 0.9)),
        StateScoped(GameState::Loading),
    ));
}
