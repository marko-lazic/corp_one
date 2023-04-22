use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::asset::asset_loading::FontAssets;
use crate::input::Cursor;
use crate::{App, GameState, Visibility};

#[derive(Component)]
struct UseMarker;

#[derive(Resource, Default)]
pub struct CursorVisibility {
    pub visible: bool,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorVisibility>();
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

        commands.spawn((
            TextBundle {
                text: Text::from_section("[E] Use", text_style)
                    .with_alignment(TextAlignment::Center),
                ..default()
            },
            UseMarker,
        ));
    }

    fn cursor_text_tooltip(
        cursor: Res<Cursor>,
        cursor_visibility: Res<CursorVisibility>,
        primary_query: Query<&Window, With<PrimaryWindow>>,
        mut query: Query<(&mut Style, &mut Visibility), With<UseMarker>>,
    ) {
        if cursor_visibility.visible {
            for (mut style, mut visibility) in &mut query {
                let Ok(primary) = primary_query.get_single() else {
                    return;
                };
                *visibility = Visibility::Visible;
                // flip the height to accommodate y going from top to bottom in UI
                let text_top = (primary.resolution.height() - cursor.screen_ui.y) + 15.0;
                let text_left = cursor.screen_ui.x + 20.0;
                style.position = UiRect {
                    top: Val::Px(text_top),
                    left: Val::Px(text_left),
                    ..Default::default()
                };
            }
        } else {
            let result = query.get_single_mut();
            if let Ok((_, mut visibility)) = result {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
