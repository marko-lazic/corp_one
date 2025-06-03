use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Init,
    Login,
    Loading,
    StarMap,
    Playing,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Loading)]
#[states(scoped_entities)]
pub enum LoadingState {
    #[default]
    Connect,
    LoadColony,
    SpawnPlayer,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<LoadingState>()
            .enable_state_scoped_entities::<GameState>();
    }
}
