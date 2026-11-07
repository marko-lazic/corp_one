use crate::prelude::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use corp_shared::prelude::*;

#[derive(Component)]
struct UiCamera;

pub struct GuiComponentsPlugin;

impl Plugin for GuiComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
            DebugGuiPlugin,
            CursorPlugin,
            LoginPlugin,
        ))
        .add_systems(Startup, setup_camera_2d)
        .add_systems(Update, update_ui_camera_clear_color)
        .add_systems(OnEnter(GameState::Init), setup_splash_screen);
    }
}
fn setup_camera_2d(mut commands: Commands) {
    commands.spawn((
        UiCamera,
        Camera2d,
        Camera {
            order: 1, // Renders after / on top of the main camera
            clear_color: ClearColorConfig::Default,
            ..default()
        },
    ));
}

fn update_ui_camera_clear_color(
    mut ui_cameras: Query<&mut Camera, (With<UiCamera>, Without<Camera3d>)>,
    world_cameras: Query<&Camera, With<Camera3d>>,
) {
    let has_world_camera = world_cameras.iter().any(|camera| camera.is_active);
    for mut camera in &mut ui_cameras {
        // During loading/login the UI camera is the only camera. It must clear
        // the background instead of loading an uninitialized render texture.
        camera.clear_color = if has_world_camera {
            ClearColorConfig::None
        } else {
            ClearColorConfig::Default
        };
    }
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
