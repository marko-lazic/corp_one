use bevy::prelude::*;

use colony::zone::ZonePlugin;

use crate::input::InputControlPlugin;
use crate::world::camera::TopDownCameraPlugin;
use crate::world::colony::ColonyPlugin;
use crate::world::player::PlayerPlugin;
use crate::world::star_map::StarMapPlugin;

pub mod camera;
pub mod character;
mod cloning;
pub mod colony;
pub mod flying_cubes;
pub mod player;
mod star_map;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::ORANGE_RED,
            brightness: 0.2,
        });
        app.add_plugin(ColonyPlugin);
        app.add_plugin(StarMapPlugin);
        app.add_plugin(InputControlPlugin);
        app.add_plugin(ZonePlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(TopDownCameraPlugin);
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum WorldSystem {
    PlayerSetup,
    CameraSetup,
    SetupInsert,
}
