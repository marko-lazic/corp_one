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
pub enum IsColonyLoaded {
    #[default]
    Running,
    Loaded,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<IsColonyLoaded>()
            .enable_state_scoped_entities::<GameState>();
    }
}
