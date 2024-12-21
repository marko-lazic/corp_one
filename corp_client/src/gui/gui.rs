use crate::{
    asset::prelude::ASSET_PATH,
    gui::{cursor_ui::CursorPlugin, debug::DebugGuiPlugin},
    state::GameState,
};
use bevy::prelude::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugGuiPlugin, CursorPlugin))
            .add_systems(OnEnter(GameState::Loading), setup.chain());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), StateScoped(GameState::Loading)));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Loading",
                TextStyle {
                    font: asset_server.load(ASSET_PATH.default_font),
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            ..Default::default()
        },
        StateScoped(GameState::Loading),
    ));
}
