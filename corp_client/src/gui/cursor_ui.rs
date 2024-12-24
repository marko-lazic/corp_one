use crate::prelude::*;
use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorUi(Vec2);

#[derive(Component)]
struct UseLabelComponent;

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
        UseLabelComponent,
        TextFont::from_font(font_assets.default_font.clone()).with_font_size(20.0),
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Center),
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
    r_hover_entities: Res<HoverEntities>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut q_use_text: Query<(&mut Node, &mut Visibility), With<UseLabelComponent>>,
) {
    let Ok((camera, gt_camera)) = camera.get_single() else {
        return;
    };

    if let Some(usable_entity) = r_hover_entities.iter().last() {
        for (mut node, mut visibility) in &mut q_use_text {
            *visibility = Visibility::Visible;
            if let Ok(hit_point_screen) =
                camera.world_to_viewport(gt_camera, usable_entity.hit_point)
            {
                node.left = Val::Px(hit_point_screen.x);
                node.top = Val::Px(hit_point_screen.y);
            }
        }
    }

    if r_hover_entities.is_empty() {
        let result = q_use_text.get_single_mut();
        if let Ok((_, mut visibility)) = result {
            *visibility = Visibility::Hidden;
        }
    }
}
