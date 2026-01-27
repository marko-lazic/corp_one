use bevy::state::state::States;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Init,
    Login,
    Connect,
    Loading,
    StarMap,
    Playing,
}
