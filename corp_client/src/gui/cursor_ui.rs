use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorUi(Vec2);

#[derive(Component)]
struct UseLabel;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorUi>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(First, update_cursor_ui_position)
            .add_systems(
                FixedUpdate,
                update_use_text.run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn((
        Text::new("[E] Use"),
        UseLabel,
        TextFont::from(font_assets.default_font.clone()).with_font_size(20.0),
        TextColor::WHITE,
        TextLayout::justify(Justify::Center),
        DespawnOnExit(GameState::Playing),
    ));
}

fn update_cursor_ui_position(q_primary_window: Single<&Window>, mut cursor: ResMut<CursorUi>) {
    if let Some(position) = q_primary_window.cursor_position() {
        cursor.x = position.x;
        cursor.y = position.y;
    }
}

fn update_use_text(
    r_hover_entities: Res<HoverEntities>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    q_use_text: Single<(&mut Node, &mut Visibility), With<UseLabel>>,
) {
    let (camera, gt_camera) = *camera;
    let (mut node, mut visibility) = q_use_text.into_inner();

    if let Some(usable_entity) = r_hover_entities.iter().last() {
        if let Ok(hit_point_screen) = camera.world_to_viewport(gt_camera, usable_entity.hit_point) {
            *visibility = Visibility::Visible;
            node.left = Val::Px(hit_point_screen.x);
            node.top = Val::Px(hit_point_screen.y);
        }
    } else if r_hover_entities.is_empty() {
        *visibility = Visibility::Hidden;
    }
}
