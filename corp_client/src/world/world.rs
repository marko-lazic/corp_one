use bevy::prelude::*;

use crate::world::{
    animator::AnimatorPlugin,
    ccc::{CameraSet, CharacterPlugin, CharacterSet, ControlPlugin, ControlSet, MainCameraPlugin},
    colony::prelude::{ColonyPlugin, ZonePlugin},
    physics::PhysicsPlugin,
    player::PlayerPlugin,
    shader::ShaderPlugin,
    star_map::StarMapPlugin,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldSystemSet {
    PlayerSetup,
    CameraSetup,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            WorldSystemSet::CameraSetup.after(WorldSystemSet::PlayerSetup),
        )
        .insert_resource(AmbientLight {
            color: bevy::color::palettes::tailwind::ORANGE_600.into(),
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
