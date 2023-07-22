use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::asset::asset_loading::FontAssets;
use crate::state::{Despawn, GameState};
use crate::{App, Visibility};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorUi(Vec2);

#[derive(Component)]
struct UseMarker;

#[derive(Resource, Default)]
pub struct CursorVisibility {
    pub visible: bool,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorVisibility>()
            .init_resource::<CursorUi>()
            .add_systems(OnEnter(GameState::LoadColony), setup)
            .add_systems(
                Update,
                cursor_text_tooltip.run_if(in_state(GameState::Playing)),
            )
            .add_systems(First, update_screen_cursor_position);
    }
}

fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {
    let text_style = TextStyle {
        font: font_assets.fira_sans.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle {
            text: Text::from_section("[E] Use", text_style).with_alignment(TextAlignment::Center),
            ..default()
        },
        UseMarker,
        Despawn,
    ));
}

fn update_screen_cursor_position(primary_query: Query<&Window>, mut cursor: ResMut<CursorUi>) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };
    if let Some(position) = primary.cursor_position() {
        cursor.x = position.x;
        cursor.y = position.y;
    }
}

fn cursor_text_tooltip(
    cursor: Res<CursorUi>,
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
            let text_top = (primary.resolution.height() - cursor.y) + 15.0;
            let text_left = cursor.x + 20.0;
            style.top = Val::Px(text_top);
            style.left = Val::Px(text_left);
        }
    } else {
        let result = query.get_single_mut();
        if let Ok((_, mut visibility)) = result {
            *visibility = Visibility::Hidden;
        }
    }
}
