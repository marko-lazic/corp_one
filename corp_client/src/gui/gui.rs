use crate::prelude::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use corp_shared::prelude::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
            DebugGuiPlugin,
            CursorPlugin,
            LoginPlugin,
        ))
        .add_systems(Startup, setup_camera_2d)
        .add_systems(OnEnter(GameState::Init), setup_splash_screen);
    }
}
fn setup_camera_2d(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,                            // Renders after / on top of the main camera
            clear_color: ClearColorConfig::None, // Don't clear, show 3D camera below
            ..default()
        },
    ));
}

fn setup_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            DespawnOnExit(GameState::Init),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading"),
                TextFont::from(asset_server.load(ASSET_PATH.default_font)).with_font_size(40.0),
                TextColor::from(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}
