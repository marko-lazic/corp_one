use crate::prelude::Colony;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Init,
    Login,
    Load(Colony),
    StarMap,
    Playing,
}

impl GameState {
    pub fn get_loading_colony(&self) -> Option<&Colony> {
        match self {
            GameState::Load(colony) => Some(colony),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Load;

impl ComputedStates for Load {
    type SourceStates = Option<GameState>;

    fn compute(sources: Option<GameState>) -> Option<Self> {
        match sources {
            Some(GameState::Load(..)) => Some(Self),
            _ => None,
        }
    }
}

#[derive(SubStates, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[source(Load = Load)]
pub enum LoadingSubState {
    #[default]
    Locate,
    ColonyLoading,
    Connect,
    SpawnPlayer,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<LoadingSubState>()
            .add_computed_state::<Load>()
            .enable_state_scoped_entities::<GameState>()
            .add_systems(OnEnter(LoadingSubState::Locate), locate);
    }
}

fn locate(
    r_state: Res<State<GameState>>,
    mut r_next_loading_sub_state: ResMut<NextState<LoadingSubState>>,
) {
    if r_state
        .get_loading_colony()
        .map(|c| c.is_star_map())
        .unwrap_or_default()
    {
        r_next_loading_sub_state.set(LoadingSubState::Connect);
    } else {
        r_next_loading_sub_state.set(LoadingSubState::ColonyLoading);
    }
}
