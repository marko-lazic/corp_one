use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use corp_shared::prelude::*;

use crate::{
    asset::FontAssets,
    gui::cursor_ui::CursorUi,
    state::{Despawn, GameState},
    world::prelude::CursorWorld,
};

#[derive(Event)]
pub enum DebugGuiEvent {
    Interaction(Entity),
}

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
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_event::<DebugGuiEvent>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    update_interaction_text,
                    update_player_position_text,
                    update_mouse_screen_position_text,
                    update_mouse_world_position_text,
                    update_camera_position_text,
                    update_player_health_text,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    commands.spawn((
        text(font_assets.default_font.clone(), 5.0, 10.0),
        InteractionText,
        Despawn,
    ));
    commands.spawn((
        text(font_assets.default_font.clone(), 25.0, 10.0),
        PlayerPositionText,
        Despawn,
    ));
    commands.spawn((
        text(font_assets.default_font.clone(), 45.0, 10.0),
        MouseScreenPositionText,
        Despawn,
    ));
    commands.spawn((
        text(font_assets.default_font.clone(), 65.0, 10.0),
        MouseWorldPositionText,
        Despawn,
    ));
    commands.spawn((
        text(font_assets.default_font.clone(), 85.0, 10.0),
        CameraDebugText,
        Despawn,
    ));
    commands.spawn((
        text(font_assets.default_font.clone(), 105.0, 10.0),
        PlayerHealthText,
        Despawn,
    ));

    let (debug_gizmo_config, _) = config_store.config_mut::<DebugGizmos>();
    debug_gizmo_config.enabled = false;
}

fn text(font: Handle<Font>, top: f32, left: f32) -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            font,
            font_size: 20.0,
            color: Color::WHITE,
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(top),
        left: Val::Px(left),
        ..default()
    })
    .with_text_justify(JustifyText::Left)
}

fn update_interaction_text(
    mut e_debug_gui: EventReader<DebugGuiEvent>,
    mut query: Query<&mut Text, With<InteractionText>>,
    q_interaction_object_type: Query<&InteractionObjectType>,
    q_name: Query<&Name>,
) {
    for event in e_debug_gui.read() {
        match event {
            DebugGuiEvent::Interaction(entity) => {
                let name = q_name
                    .get(entity.clone())
                    .map(|n| n.as_str())
                    .unwrap_or("unknown");

                let interaction_type = q_interaction_object_type
                    .get(entity.clone())
                    .map(|o| format!("{o:?}"))
                    .unwrap_or("unknown".into());

                let message =
                    format!("Entity {entity:?}, Name {name}, Interaction Type {interaction_type}");

                for mut interaction_text in query.iter_mut() {
                    interaction_text.sections[0].value = message.to_owned();
                }
            }
        }
    }
}

fn update_player_position_text(
    q_transform: Query<&Transform, With<Player>>,
    mut query: Query<&mut Text, With<PlayerPositionText>>,
) {
    let Ok(player_pos) = q_transform.get_single().map(|t| t.translation) else {
        return;
    };
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Player {:.0} {:.0} {:.0}",
            player_pos.x, player_pos.y, player_pos.z
        );
    }
}

fn update_mouse_screen_position_text(
    cursor: Res<CursorUi>,
    mut screen_text: Query<&mut Text, With<MouseScreenPositionText>>,
) {
    for mut text in screen_text.iter_mut() {
        text.sections[0].value = format!("MS Screen {:.0} {:.0}", cursor.x, cursor.y);
    }
}

fn update_mouse_world_position_text(
    cursor: Res<CursorWorld>,
    mut world_text: Query<&mut Text, With<MouseWorldPositionText>>,
) {
    for mut text in world_text.iter_mut() {
        text.sections[0].value =
            format!("MS World {:.0} {:.0} {:.0}", cursor.x, cursor.y, cursor.z);
    }
}

fn update_camera_position_text(
    q_camera_pos: Query<&Transform, With<Camera>>,
    mut query: Query<&mut Text, With<CameraDebugText>>,
) {
    let Ok(cam_pos) = q_camera_pos.get_single().map(|t| t.translation) else {
        return;
    };
    for mut text in query.iter_mut() {
        text.sections[0].value =
            format!("Camera {:.0} {:.0} {:.0}", cam_pos.x, cam_pos.y, cam_pos.z);
    }
}

fn update_player_health_text(
    q_health: Query<&Health, With<Player>>,
    mut q_text: Query<&mut Text, With<PlayerHealthText>>,
) {
    let Ok(health) = q_health.get_single() else {
        return;
    };
    for mut text in q_text.iter_mut() {
        text.sections[0].value = format!("Health {:.0}", health.get_health());
    }
}
