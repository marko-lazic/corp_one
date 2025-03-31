use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    StarMap,
    LoadColony,
    Playing,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::LoadColony)]
pub enum LoadingSubState {
    #[default]
    Loading,
    Loaded,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<LoadingSubState>()
            .enable_state_scoped_entities::<GameState>();
    }
}
