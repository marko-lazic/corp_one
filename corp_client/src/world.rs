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
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ColonyPlugin);
        app.add_plugin(StarMapPlugin);
        app.add_plugin(InputControlPlugin);
        app.add_plugin(ZonePlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(TopDownCameraPlugin);
    }
}
