use crate::{gui::cursor_ui::CursorUi, prelude::*};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use corp_shared::prelude::*;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DebugGizmos {}

#[derive(Component)]
struct InteractionText;

#[derive(Component)]
struct PlayerPositionText;

#[derive(Component)]
struct MouseScreenPositionText;

#[derive(Component)]
struct MouseWorldPositionText;

#[derive(Component)]
struct CameraDebugText;

#[derive(Component)]
struct PlayerHealthText;

pub struct DebugGuiPlugin;

impl Plugin for DebugGuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<DebugGizmos>()
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                FixedUpdate,
                (
                    update_player_position_text,
                    update_mouse_screen_position_text,
                    update_mouse_world_position_text,
                    update_camera_position_text,
                    update_player_health_text,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(update_interaction_text);
    }
}

fn setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    commands.spawn((
        DebugTextBundle::new(font_assets.default_font.clone(), 5.0, 10.0),
        InteractionText,
        StateScoped(GameState::Playing),
    ));
    commands.spawn((
        DebugTextBundle::new(font_assets.default_font.clone(), 25.0, 10.0),
        PlayerPositionText,
        StateScoped(GameState::Playing),
    ));
    commands.spawn((
        DebugTextBundle::new(font_assets.default_font.clone(), 45.0, 10.0),
        MouseScreenPositionText,
        StateScoped(GameState::Playing),
    ));
    commands.spawn((
        DebugTextBundle::new(font_assets.default_font.clone(), 65.0, 10.0),
        MouseWorldPositionText,
        StateScoped(GameState::Playing),
    ));
    commands.spawn((
        DebugTextBundle::new(font_assets.default_font.clone(), 85.0, 10.0),
        CameraDebugText,
        StateScoped(GameState::Playing),
    ));
    commands.spawn((
        DebugTextBundle::new(font_assets.default_font.clone(), 105.0, 10.0),
        PlayerHealthText,
        StateScoped(GameState::Playing),
    ));

    let (debug_gizmo_config, _) = config_store.config_mut::<DebugGizmos>();
    debug_gizmo_config.enabled = false;
}

#[derive(Bundle)]
struct DebugTextBundle {
    text: Text,
    node: Node,
    text_color: TextColor,
    text_font: TextFont,
    text_layout: TextLayout,
}

impl DebugTextBundle {
    pub fn new(font: Handle<Font>, top: f32, left: f32) -> Self {
        DebugTextBundle {
            text: Text::new(""),
            node: Node {
                top: Val::Px(top),
                left: Val::Px(left),
                ..default()
            },
            text_color: TextColor::WHITE,
            text_font: TextFont::from_font(font),
            text_layout: TextLayout::new_with_justify(JustifyText::Left),
        }
    }
}

fn update_interaction_text(
    trigger: Trigger<UsableTarget>,
    interaction_text_entity: Single<Entity, With<InteractionText>>,
    q_name: Query<&Name>,
    mut writer: TextUiWriter,
) {
    let usable_entity = trigger.entity;
    let name = q_name
        .get(usable_entity)
        .map(|n| n.as_str())
        .unwrap_or("unknown");
    let message = format!("Entity {usable_entity:?}, Name {name}");
    *writer.text(*interaction_text_entity, 0) = message.to_owned();
}

fn update_player_position_text(
    q_player_tr: Single<&Transform, With<LocalPlayer>>,
    player_position_text_entity: Single<Entity, With<PlayerPositionText>>,
    mut writer: TextUiWriter,
) -> Result {
    let player_pos = q_player_tr.translation;
    *writer.text(*player_position_text_entity, 0) = format!(
        "Player {:.0} {:.0} {:.0}",
        player_pos.x, player_pos.y, player_pos.z
    );
    Ok(())
}

fn update_mouse_screen_position_text(
    r_cursor: Res<CursorUi>,
    mouse_screen_position_text_entity: Single<Entity, With<MouseScreenPositionText>>,
    mut writer: TextUiWriter,
) {
    *writer.text(*mouse_screen_position_text_entity, 0) =
        format!("MS Screen {:.0} {:.0}", r_cursor.x, r_cursor.y);
}

fn update_mouse_world_position_text(
    r_cursor: Res<CursorWorld>,
    mouse_world_position_text_entity: Single<Entity, With<MouseWorldPositionText>>,
    mut writer: TextUiWriter,
) {
    *writer.text(*mouse_world_position_text_entity, 0) = format!(
        "MS World {:.0} {:.0} {:.0}",
        r_cursor.x, r_cursor.y, r_cursor.z
    );
}

fn update_camera_position_text(
    q_camera_pos: Single<&Transform, With<Camera>>,
    camera_debug_text_entity: Single<Entity, With<CameraDebugText>>,
    mut writer: TextUiWriter,
) {
    let cam_pos = q_camera_pos.translation;
    *writer.text(*camera_debug_text_entity, 0) =
        format!("Camera {:.0} {:.0} {:.0}", cam_pos.x, cam_pos.y, cam_pos.z);
}

fn update_player_health_text(
    q_player_health: Single<&Health, With<LocalPlayer>>,
    player_health_text_entity: Single<Entity, With<PlayerHealthText>>,
    mut writer: TextUiWriter,
) {
    let health = q_player_health.get_health();
    *writer.text(*player_health_text_entity, 0) = format!("Health {:.0}", health);
}
