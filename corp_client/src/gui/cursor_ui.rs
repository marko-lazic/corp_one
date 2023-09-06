use bevy::{prelude::*, text::DEFAULT_FONT_HANDLE};

use crate::{
    state::{Despawn, GameState},
    world::prelude::UseEntity,
};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorUi(Vec2);

#[derive(Component)]
struct UseText;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorUi>()
            .add_systems(OnEnter(GameState::LoadColony), setup)
            .add_systems(First, update_cursor_ui_position)
            .add_systems(Update, update_use_text.run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "[E] Use",
            TextStyle {
                font: DEFAULT_FONT_HANDLE.typed(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center),
        UseText,
        Despawn,
    ));
}

fn update_cursor_ui_position(primary_query: Query<&Window>, mut cursor: ResMut<CursorUi>) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };
    if let Some(position) = primary.cursor_position() {
        cursor.x = position.x;
        cursor.y = position.y;
    }
}

fn update_use_text(
    cursor: Res<CursorUi>,
    r_use_target: Res<UseEntity>,
    mut q_use_text: Query<(&mut Style, &mut Visibility), With<UseText>>,
) {
    if r_use_target.get().is_some() {
        for (mut style, mut visibility) in &mut q_use_text {
            *visibility = Visibility::Visible;
            let text_top = cursor.y + 15.0;
            let text_left = cursor.x + 20.0;
            style.top = Val::Px(text_top);
            style.left = Val::Px(text_left);
        }
    } else {
        let result = q_use_text.get_single_mut();
        if let Ok((_, mut visibility)) = result {
            *visibility = Visibility::Hidden;
        }
    }
}
