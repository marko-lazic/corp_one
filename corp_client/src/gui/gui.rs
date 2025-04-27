use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugGuiPlugin, CursorPlugin))
            .add_systems(OnEnter(GameState::Loading), loading_splash);
    }
}

fn loading_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, StateScoped(GameState::Loading)));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            StateScoped(GameState::Loading),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading"),
                TextFont::from_font(asset_server.load(ASSET_PATH.default_font))
                    .with_font_size(40.0),
                TextColor::from(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}
