use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use corp_shared::prelude::Health;

use crate::input::Cursor;
use crate::state::Despawn;
use crate::world::camera::TopDownCamera;
use crate::world::player::Player;
use crate::Game;
use crate::GameState;

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
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_system(Self::setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (
                Self::fps_update,
                Self::player_position_update,
                Self::mouse_screen_position_update,
                Self::mouse_world_position_update,
                Self::camera_metrics,
                Self::camera_debug_text,
                Self::player_health_metric,
            )
                .chain()
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

impl MetricsPlugin {
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

    fn fps_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
        for mut text in query.iter_mut() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.sections[0].value = format!("FPS {:.0}", average);
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
            text.sections[0].value =
                format!("Player {:.0} {:.0} {:.0}", player_x, player_y, player_z);
        }
    }

    fn mouse_screen_position_update(
        cursor: Res<Cursor>,
        mut screen_text: Query<&mut Text, With<MouseScreenPositionText>>,
    ) {
        let cs_x = &cursor.screen_ui.x;
        let cs_y = &cursor.screen_ui.y;
        for mut text in screen_text.iter_mut() {
            text.sections[0].value = format!("MS Screen {:.0} {:.0}", cs_x, cs_y);
        }
    }

    fn mouse_world_position_update(
        cursor: Res<Cursor>,
        mut world_text: Query<&mut Text, With<MouseWorldPositionText>>,
    ) {
        let ws_x = &cursor.world.x;
        let ws_y = &cursor.world.y;
        let ws_z = &cursor.world.z;
        for mut text in world_text.iter_mut() {
            text.sections[0].value = format!("MS World {:.0} {:.0} {:.0}", ws_x, ws_y, ws_z);
        }
    }

    fn camera_metrics(
        mut game: ResMut<Game>,
        mut query: Query<(&mut Transform, &mut TopDownCamera, &mut Camera)>,
    ) {
        for (transform, _, _) in query.iter_mut() {
            game.camera_transform = Some(*transform);
        }
    }

    fn camera_debug_text(game: Res<Game>, mut query: Query<&mut Text, With<CameraDebugText>>) {
        if let Some(transform) = game.camera_transform {
            for mut text in query.iter_mut() {
                let vec3 = transform.translation;
                text.sections[0].value =
                    format!("Camera {:.0} {:.0} {:.0}", vec3.x, vec3.y, vec3.z);
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
            position: UiRect {
                top: Val::Px(top),
                left: Val::Px(left),
                ..Default::default()
            },
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        }
    }
}
