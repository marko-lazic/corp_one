use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::AppLooplessStateExt;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::input::Cursor;
use crate::world::camera::TopDownCamera;
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
struct PlayerHealth;

pub struct MetricsPlugin;

impl MetricsPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn_bundle(metrics_utils::label(5.0, 10.0, &asset_server))
            .insert(FpsText);
        commands
            .spawn_bundle(metrics_utils::label(25.0, 10.0, &asset_server))
            .insert(PlayerPositionText);
        commands
            .spawn_bundle(metrics_utils::label(45.0, 10.0, &asset_server))
            .insert(MouseScreenPositionText);
        commands
            .spawn_bundle(metrics_utils::label(65.0, 10.0, &asset_server))
            .insert(MouseWorldPositionText);
        commands
            .spawn_bundle(metrics_utils::label(85.0, 10.0, &asset_server))
            .insert(CameraDebugText);
        commands
            .spawn_bundle(metrics_utils::label(105.0, 10.0, &asset_server))
            .insert(PlayerHealth);
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
        let cs_x = &cursor.screen.x;
        let cs_y = &cursor.screen.y;
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
            game.camera_transform = Some(transform.clone());
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
        mut query: Query<&mut Text, With<PlayerHealth>>,
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

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_enter_system(GameState::Playing, Self::setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::fps_update)
                .with_system(Self::player_position_update)
                .with_system(Self::mouse_screen_position_update)
                .with_system(Self::mouse_world_position_update)
                .with_system(Self::camera_metrics)
                .with_system(Self::camera_debug_text)
                .with_system(Self::player_health_metric)
                .into(),
        );
    }
}

mod metrics_utils {
    use bevy::prelude::*;

    pub fn label(top: f32, left: f32, asset_server: &Res<AssetServer>) -> TextBundle {
        TextBundle {
            style: default_style(top, left),
            text: default_text(&asset_server),
            ..Default::default()
        }
    }

    fn default_text(asset_server: &Res<AssetServer>) -> Text {
        let font_handle = asset_server.load("fonts/FiraMono-Medium.ttf");
        Text::with_section(
            "Corp One".to_string(),
            TextStyle {
                font: font_handle,
                font_size: 20.0,
                color: Color::WHITE,
            },
            TextAlignment::default(),
        )
    }

    fn default_style(top: f32, left: f32) -> Style {
        Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(top),
                left: Val::Px(left),
                ..Default::default()
            },
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        }
    }
}
