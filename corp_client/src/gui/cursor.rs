use bevy::app::Plugin;
use bevy::prelude::*;

use crate::asset::asset_loading::FontAssets;
use crate::input::Cursor;
use crate::{App, GameState, UiRect, Visibility};

#[derive(Component)]
struct CursorText;

#[derive(Resource, Default)]
pub struct CursorInfo {
    pub show_use: bool,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorInfo>();
        app.add_system(Self::setup.in_schedule(OnEnter(GameState::LoadColony)));
        app.add_system(Self::cursor_text_tooltip.in_set(OnUpdate(GameState::Playing)));
    }
}

impl CursorPlugin {
    fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {
        let text_style = TextStyle {
            font: font_assets.fira_sans.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        };

        commands
            .spawn(TextBundle {
                text: Text::from_section("[E] Use", text_style)
                    .with_alignment(TextAlignment::Center),
                ..Default::default()
            })
            .insert(CursorText);
    }

    fn cursor_text_tooltip(
        cursor: Res<Cursor>,
        cursor_info: Res<CursorInfo>,
        mut query: Query<(&mut Style, &mut Visibility), (With<Text>, With<CursorText>)>,
    ) {
        if cursor_info.show_use {
            let mouse_screen_x = cursor.screen.x;
            let mouse_screen_y = cursor.screen.y;
            let result = query.get_single_mut();
            if let Ok((mut style, mut visibility)) = result {
                *visibility = Visibility::Visible;
                style.position = UiRect {
                    top: Val::Px(-mouse_screen_y + 20.0),
                    left: Val::Px(mouse_screen_x + 15.0),
                    ..Default::default()
                }
            }
        } else {
            let result = query.get_single_mut();
            if let Ok((_, mut visibility)) = result {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
