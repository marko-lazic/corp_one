use std::time::Duration;

use bevy_app::{prelude::*, PluginGroupBuilder, ScheduleRunnerPlugin};
use bevy_log::LogPlugin;

use crate::database::DbPlugin;

mod database;

fn main() {
    let frames_per_second = Duration::from_secs_f64(1.0 / 60.0);

    App::new()
        .add_plugins((
            CorpBevyPlugin.set(ScheduleRunnerPlugin::run_loop(frames_per_second)),
            bevy_tokio_tasks::TokioTasksPlugin::default(),
            DbPlugin,
        ))
        .run();
}

pub struct CorpBevyPlugin;

impl PluginGroup for CorpBevyPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy_core::TaskPoolPlugin::default())
            .add(bevy_core::TypeRegistrationPlugin::default())
            .add(bevy_core::FrameCountPlugin::default())
            .add(bevy_time::TimePlugin::default())
            .add(bevy_app::ScheduleRunnerPlugin::default())
            .add(LogPlugin::default())
    }
}
