use crate::world::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::{ActionPlugin, StructurePlugin};

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
            WorldPhysicsPlugin,
            AnimatorPlugin,
            StarMapPlugin,
            ControlPlugin,
            MainCameraPlugin,
            PlayerPlugin,
            CloningLocalPlugin,
            VortexPlugin,
            BarrierPlugin,
            StructurePlugin,
            AreaPlugin,
            ActionPlugin,
        ));
    }
}
