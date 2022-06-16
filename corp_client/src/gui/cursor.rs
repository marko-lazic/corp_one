use bevy::app::Plugin;
use bevy::prelude::{
    Color, Commands, Component, HorizontalAlign, Query, Res, Style, Text, TextAlignment,
    TextBundle, TextStyle, Val, With,
};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::asset::asset_loading::FontAssets;
use crate::input::Cursor;
use crate::{App, GameState, Rect, Visibility};

pub struct CursorPlugin;

#[derive(Default)]
pub struct CursorInfo {
    pub show_use: bool,
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorInfo>();
        app.add_enter_system(GameState::LoadColony, Self::setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::cursor_text_tooltip)
                .into(),
        );
    }
}

impl CursorPlugin {
    fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {
        let text_style = TextStyle {
            font: font_assets.fira_sans.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        };

        commands
            .spawn_bundle(TextBundle {
                text: Text::with_section("[E] Use", text_style, text_alignment),
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
                visibility.is_visible = true;
                style.position = Rect {
                    top: Val::Px(-mouse_screen_y + 20.0),
                    left: Val::Px(mouse_screen_x + 15.0),
                    ..Default::default()
                }
            }
        } else {
            let result = query.get_single_mut();
            if let Ok((_, mut visibility)) = result {
                visibility.is_visible = false;
            }
        }
    }
}

#[derive(Component)]
struct CursorText;
