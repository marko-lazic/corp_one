use bevy::prelude::*;

use corp_shared::prelude::CLONE_HEALTH;

use crate::asset::asset_loading::ColonyAssets;
use crate::constants::state::GameState;
use crate::Game;

pub struct StarMapPlugin;

impl StarMapPlugin {
    fn vort_dead_player_to_cloning(
        mut game: ResMut<Game>,
        mut game_state: ResMut<State<GameState>>,
        colony_assets: Res<ColonyAssets>,
    ) {
        if game.health.is_dead() {
            game.current_colony_asset = colony_assets.cloning.clone();
            game.health.set_hit_points(CLONE_HEALTH);
            let _result = game_state.set(GameState::Playing);
        }
    }
}

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::StarMap)
                .with_system(Self::vort_dead_player_to_cloning.system()),
        );
    }
}
