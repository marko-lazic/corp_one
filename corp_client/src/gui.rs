use bevy::app::Plugin;
use bevy::prelude::{Commands, UiCameraBundle};
use iyes_loopless::condition::ConditionSet;

pub use gui::cursor::CursorInfo;

use crate::gui::cursor::CursorPlugin;
use crate::{gui, App, GameState, MetricsPlugin};

mod cursor;
pub mod metrics;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_ui_camera);
        app.add_plugin(MetricsPlugin);
        app.add_plugin(CursorPlugin);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::SpawnPlayer)
                .with_system(Self::setup_ui_camera)
                .into(),
        );
    }
}

impl GuiPlugin {
    pub fn setup_ui_camera(mut commands: Commands) {
        commands.spawn_bundle(UiCameraBundle::default());
    }
}
