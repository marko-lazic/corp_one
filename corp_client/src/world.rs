use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_mod_bounding::{aabb, BoundingVolumePlugin};
use bevy_mod_raycast::DefaultRaycastingPlugin;

use colony::zone::ZonePlugin;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::input::input_command::PlayerAction;
use crate::input::InputControlPlugin;
use crate::world::camera::TopDownCameraPlugin;
use crate::world::colony::ColonyPlugin;
use crate::world::cursor::MyRaycastSet;
use crate::world::player::PlayerPlugin;
use crate::world::star_map::StarMapPlugin;

pub mod camera;
pub mod character;
mod cloning;
pub mod colony;
mod cursor;
pub mod flying_cubes;
pub mod player;
mod player_bundle;
pub mod scene;
mod star_map;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerAction>();
        // app.add_plugin(ScenePlugin);
        app.add_plugin(ColonyPlugin);
        app.add_plugin(StarMapPlugin);
        app.add_plugin(InputControlPlugin);
        app.add_plugin(BoundingVolumePlugin::<aabb::Aabb>::default());
        app.add_plugin(ZonePlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(TopDownCameraPlugin);
        app.add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default());
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(cursor::update_raycast_with_cursor.system()),
        );
    }
}
