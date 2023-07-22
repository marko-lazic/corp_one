use bevy::log::info;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    StarMap,
    LoadColony,
    SpawnPlayer,
    Playing,
}

#[derive(Component)]
pub struct Despawn;

#[derive(Component)]
pub struct Persistent;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnExit(GameState::Loading), teardown)
            .add_systems(OnExit(GameState::Playing), teardown)
            .add_systems(OnExit(GameState::StarMap), teardown);
    }
}

fn teardown(
    game_state: Res<State<GameState>>,
    mut commands: Commands,
    entities: Query<Entity, With<Despawn>>,
) {
    info!("Teardown {:?}", game_state);
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
