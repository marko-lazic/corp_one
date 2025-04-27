use crate::prelude::ASSET_PATH;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct LoginScreenPlugin;

impl Plugin for LoginScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Login), setup_login_screen);
    }
}

fn setup_login_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, StateScoped(GameState::Init)));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            StateScoped(GameState::Init),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Login"),
                TextFont::from_font(asset_server.load(ASSET_PATH.default_font))
                    .with_font_size(40.0),
                TextColor::from(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}
