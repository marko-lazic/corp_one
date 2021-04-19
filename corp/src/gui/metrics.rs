use crate::world::player::Player;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct MetricsPlugin;

struct FpsText;

struct PlayerPositionText;

struct MousePositionText;

pub struct MouseRes(pub Vec2);

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup.system())
            .add_system(fps_update.system())
            .add_system(player_position_update.system())
            .add_system(mouse_screen_position_update.system())
            .add_system(update_pos.system());
    }
}

fn update_pos(mut mouse_loc: ResMut<MouseRes>, mut cursor_moved_events: EventReader<CursorMoved>) {
    for event in cursor_moved_events.iter() {
        mouse_loc.0 = event.position;
    }
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
            "Player position: X: ({:.1}) Y: ({:.1}) Z: ({:.1})",
            player_x, player_y, player_z
        );
    }
}

fn mouse_screen_position_update(
    mouse_pos: Res<MouseRes>,
    mut query: Query<&mut Text, With<MousePositionText>>,
) {
    let mouse_screen_x = mouse_pos.0.x;
    let mouse_screen_y = mouse_pos.0.y;
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Mouse screen position: X: ({:.1}) Y: ({:.1})",
            mouse_screen_x, mouse_screen_y
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MouseRes(Vec2::ZERO));

    commands
        .spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: default_style(5.0, 10.0),
            text: default_text(&asset_server),
            ..Default::default()
        })
        .insert(FpsText);

    commands
        .spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: default_style(25.0, 10.0),
            text: default_text(&asset_server),
            ..Default::default()
        })
        .insert(PlayerPositionText);

    commands
        .spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: default_style(45.0, 10.0),
            text: default_text(&asset_server),
            ..Default::default()
        })
        .insert(MousePositionText);
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
