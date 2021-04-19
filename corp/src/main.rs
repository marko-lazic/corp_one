mod audio;
mod gui;
mod world;

//use audio::live::LivePlugin;
use crate::world::agency::input::InputPlugin;
use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::prelude::*;
use gui::{console::ConsolePlugin, metrics::MetricsPlugin};
use world::{player::PlayerPlugin, scene::ScenePlugin};

static CORP_ONE_GAME_TITLE: &str = "Corp One";

#[derive(Default)]
struct Game {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum GameSate {
    #[allow(dead_code)]
    StarMap,
    InWorld,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum SystemLoading {
    Scene,
    PlayerSetup,
}

fn main() {
    App::build()
        .insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: CORP_ONE_GAME_TITLE.to_string(),
            width: 1600.0,
            height: 1600.0,
            ..Default::default()
        })
        // .add_startup_stage(GAME_SETUP_STARTUP_STAGE)
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_state(GameSate::InWorld)
        .add_plugin(ConsolePlugin)
        // .add_plugin(LivePlugin)
        .add_plugin(MetricsPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
