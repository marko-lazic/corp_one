use crate::world::prelude::*;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldSystemSet {
    PlayerSetup,
    CameraSetup,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            WorldSystemSet::CameraSetup.after(WorldSystemSet::PlayerSetup),
        )
        .insert_resource(AmbientLight {
            color: bevy::color::palettes::tailwind::ORANGE_600.into(),
            ..default()
        })
        .add_plugins((
            ShaderPlugin,
            WorldPhysicsPlugin,
            ColonyPlugin,
            AnimatorPlugin,
            StarMapPlugin,
            CharacterPlugin,
            ControlPlugin,
            MainCameraPlugin,
            ZonePlugin,
            PlayerPlugin,
            CloningPlugin,
        ))
        .configure_sets(
            FixedUpdate,
            (
                ControlSet::PlayingInput.before(CharacterSet::Movement),
                CameraSet::Update.after(CharacterSet::Movement),
            ),
        );
    }
}
