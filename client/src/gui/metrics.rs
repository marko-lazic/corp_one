use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::constants::tick;
use crate::world::camera::TopDownCamera;
use crate::world::player::Player;
use crate::Game;
use common::mouse::MousePosWorld;

pub struct Metrics {
    pub mouse_screen_position: Vec2,
    pub mouse_world_position: Vec3,
}

struct FpsText;
struct PlayerPositionText;
struct MouseScreenPositionText;
struct MouseWorldPositionText;
struct CameraDebugText;

pub struct MetricsPlugin;

impl MetricsPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(Metrics {
            mouse_screen_position: Vec2::ZERO,
            mouse_world_position: Vec3::ZERO,
        });

        commands.spawn_bundle(UiCameraBundle::default());

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
    }

    fn fps_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
        for mut text in query.iter_mut() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.sections[0].value = format!("FPS: {:.2}", average);
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
            text.sections[0].value = format!(
                "Player position: X: {:.1} Y: {:.1} Z: {:.1}",
                player_x, player_y, player_z
            );
        }
    }

    fn mouse_screen_position_update(
        metrics: Res<Metrics>,
        mut query: Query<&mut Text, With<MouseScreenPositionText>>,
    ) {
        let mouse_screen_x = metrics.mouse_screen_position.x;
        let mouse_screen_y = metrics.mouse_screen_position.y;
        for mut text in query.iter_mut() {
            text.sections[0].value = format!(
                "Mouse screen position: X: {:.1} Y: {:.1}",
                mouse_screen_x, mouse_screen_y
            );
        }
    }

    fn mouse_world_position_update(
        mouse: Res<MousePosWorld>,
        mut query: Query<&mut Text, With<MouseWorldPositionText>>,
    ) {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Mouse world position: {}", *mouse);
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
                text.sections[0].value = format!("Cam {:?}", transform.translation);
            }
        }
    }
}

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(Self::setup.system());
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::fps_update.system())
                .with_system(Self::player_position_update.system())
                .with_system(Self::mouse_screen_position_update.system())
                .with_system(Self::mouse_world_position_update.system())
                .with_system(Self::camera_metrics.system())
                .with_system(Self::camera_debug_text.system()),
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
            "FPS".to_string(),
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
