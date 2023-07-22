use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use corp_shared::prelude::Health;
use corp_shared::prelude::*;

use crate::gui::cursor_ui::CursorUi;
use crate::state::Despawn;
use crate::state::GameState;
use crate::world::prelude::CursorWorld;
use crate::Game;

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

pub struct MetricsPlugin;

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    fps_update,
                    player_position_update,
                    mouse_screen_position_update,
                    mouse_world_position_update,
                    camera_metrics,
                    camera_debug_text,
                    player_health_metric,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        metrics_utils::label(5.0, 10.0, &asset_server),
        FpsText,
        Despawn,
    ));
    commands.spawn((
        metrics_utils::label(25.0, 10.0, &asset_server),
        PlayerPositionText,
        Despawn,
    ));
    commands.spawn((
        metrics_utils::label(45.0, 10.0, &asset_server),
        MouseScreenPositionText,
        Despawn,
    ));
    commands.spawn((
        metrics_utils::label(65.0, 10.0, &asset_server),
        MouseWorldPositionText,
        Despawn,
    ));
    commands.spawn((
        metrics_utils::label(85.0, 10.0, &asset_server),
        CameraDebugText,
        Despawn,
    ));
    commands.spawn((
        metrics_utils::label(105.0, 10.0, &asset_server),
        PlayerHealthText,
        Despawn,
    ));
}

fn fps_update(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(smoothed) = fps.smoothed() {
                text.sections[0].value = format!("FPS {:.0}", smoothed);
            }
        }
    }
}

fn player_position_update(
    mut player_position: Query<(&Player, &mut Transform)>,
    mut query: Query<&mut Text, With<PlayerPositionText>>,
) {
    let mut player_x = 0f32;
    let mut player_y = 0f32;
    let mut player_z = 0f32;
    for (_player, transform) in player_position.iter_mut() {
        player_x = transform.translation.x;
        player_y = transform.translation.y;
        player_z = transform.translation.z;
    }
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Player {:.0} {:.0} {:.0}", player_x, player_y, player_z);
    }
}

fn mouse_screen_position_update(
    cursor: Res<CursorUi>,
    mut screen_text: Query<&mut Text, With<MouseScreenPositionText>>,
) {
    let cs_x = &cursor.x;
    let cs_y = &cursor.y;
    for mut text in screen_text.iter_mut() {
        text.sections[0].value = format!("MS Screen {:.0} {:.0}", cs_x, cs_y);
    }
}

fn mouse_world_position_update(
    cursor: Res<CursorWorld>,
    mut world_text: Query<&mut Text, With<MouseWorldPositionText>>,
) {
    let ws_x = &cursor.x;
    let ws_y = &cursor.y;
    let ws_z = &cursor.z;
    for mut text in world_text.iter_mut() {
        text.sections[0].value = format!("MS World {:.0} {:.0} {:.0}", ws_x, ws_y, ws_z);
    }
}

fn camera_metrics(mut game: ResMut<Game>, mut query: Query<&mut Transform, With<Camera>>) {
    for transform in query.iter_mut() {
        game.camera_transform = Some(*transform);
    }
}

fn camera_debug_text(game: Res<Game>, mut query: Query<&mut Text, With<CameraDebugText>>) {
    if let Some(transform) = game.camera_transform {
        for mut text in query.iter_mut() {
            let vec3 = transform.translation;
            text.sections[0].value = format!("Camera {:.0} {:.0} {:.0}", vec3.x, vec3.y, vec3.z);
        }
    }
}

fn player_health_metric(
    game: Res<Game>,
    healths: Query<&Health>,
    mut query: Query<&mut Text, With<PlayerHealthText>>,
) {
    if let Some(entity) = game.player_entity {
        if let Ok(health) = healths.get(entity) {
            for mut text in query.iter_mut() {
                text.sections[0].value = format!("Health {:.0}", health.get_health());
            }
        }
    }
}

mod metrics_utils {
    use bevy::prelude::*;

    pub fn label(top: f32, left: f32, asset_server: &Res<AssetServer>) -> TextBundle {
        TextBundle {
            style: default_style(top, left),
            text: default_text(asset_server),
            ..Default::default()
        }
    }

    fn default_text(asset_server: &Res<AssetServer>) -> Text {
        let font_handle = asset_server.load("fonts/FiraMono-Medium.ttf");
        Text::from_section(
            "Corp One".to_string(),
            TextStyle {
                font: font_handle,
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::Left)
    }

    fn default_style(top: f32, left: f32) -> Style {
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(top),
            left: Val::Px(left),
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        }
    }
}
