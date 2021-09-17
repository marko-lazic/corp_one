use bevy::prelude::*;

use crate::constants::state::GameState;

pub struct StarMapPlugin;

impl StarMapPlugin {
    fn setup_starmap() {
        info!("Entering Starmap state");
    }
}

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::StarMap).with_system(Self::setup_starmap.system()),
        );
    }
}
