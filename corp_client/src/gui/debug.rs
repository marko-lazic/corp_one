use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::DEFAULT_FONT_HANDLE,
};

use corp_shared::prelude::*;

use crate::{
    gui::cursor_ui::CursorUi,
    state::{Despawn, GameState},
    world::prelude::CursorWorld,
};

#[derive(Component)]
struct FpsText;

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
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    update_fps_text,
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

fn setup(mut commands: Commands) {
    commands.spawn((text(5.0, 10.0), FpsText, Despawn));
    commands.spawn((text(25.0, 10.0), PlayerPositionText, Despawn));
    commands.spawn((text(45.0, 10.0), MouseScreenPositionText, Despawn));
    commands.spawn((text(65.0, 10.0), MouseWorldPositionText, Despawn));
    commands.spawn((text(85.0, 10.0), CameraDebugText, Despawn));
    commands.spawn((text(105.0, 10.0), PlayerHealthText, Despawn));
}

fn text(top: f32, left: f32) -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            font: DEFAULT_FONT_HANDLE.typed(),
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
    .with_text_alignment(TextAlignment::Left)
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .map(|f| f.smoothed())
        .flatten()
    else {
        return;
    };
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("FPS {:.0}", fps);
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
