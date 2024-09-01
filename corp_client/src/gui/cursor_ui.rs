use bevy::prelude::*;

use crate::{asset::FontAssets, state::GameState, world::prelude::UseEntity};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorUi(Vec2);

#[derive(Component)]
struct UseText;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorUi>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(First, update_cursor_ui_position)
            .add_systems(Update, update_use_text.run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn((
        TextBundle::from_section(
            "[E] Use",
            TextStyle {
                font: font_assets.default_font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_text_justify(JustifyText::Center),
        UseText,
        StateScoped(GameState::Playing),
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
    r_use_entity: Res<UseEntity>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut q_use_text: Query<(&mut Style, &mut Visibility), With<UseText>>,
) {
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };
    if let Some(usable_entity) = r_use_entity.0.iter().last() {
        let (_, hit_point) = usable_entity.get();
        for (mut style, mut visibility) in &mut q_use_text {
            *visibility = Visibility::Visible;
            if let Some(hit_point_screen) = camera.world_to_viewport(camera_transform, hit_point) {
                style.left = Val::Px(hit_point_screen.x);
                style.top = Val::Px(hit_point_screen.y);
            }
        }
    }

    if r_use_entity.0.is_empty() {
        let result = q_use_text.get_single_mut();
        if let Ok((_, mut visibility)) = result {
            *visibility = Visibility::Hidden;
        }
    }
}
