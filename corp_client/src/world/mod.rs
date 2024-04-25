use bevy::prelude::*;

use crate::world::{
    animator::AnimatorPlugin,
    ccc::{CameraSet, CharacterPlugin, CharacterSet, ControlPlugin, ControlSet, MainCameraPlugin},
    colony::{prelude::ColonyPlugin, zone::ZonePlugin},
    physics::PhysicsPlugin,
    player::PlayerPlugin,
    shader::ShaderPlugin,
    star_map::StarMapPlugin,
    WorldSystemSet::{CameraSetup, PlayerSetup},
};

mod animator;
mod ccc;
mod cloning;
pub mod colony;
mod physics;
pub mod player;
mod shader;
mod star_map;

pub mod prelude {
    pub use super::{
        ccc::{CharacterMovement, CursorWorld, UseEntity},
        colony::*,
        physics::*,
        player::*,
    };
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldSystemSet {
    PlayerSetup,
    CameraSetup,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, CameraSetup.after(PlayerSetup))
            .insert_resource(AmbientLight {
                color: Color::ORANGE_RED,
                ..default()
            })
            .add_plugins((
                ShaderPlugin,
                PhysicsPlugin,
                ColonyPlugin,
                AnimatorPlugin,
                StarMapPlugin,
                CharacterPlugin,
                ControlPlugin,
                MainCameraPlugin,
                ZonePlugin,
                PlayerPlugin,
            ))
            .configure_sets(
                Update,
                (
                    ControlSet::PlayingInput.before(CharacterSet::Movement),
                    CameraSet::Update.after(CharacterSet::Movement),
                ),
            );
    }
}
