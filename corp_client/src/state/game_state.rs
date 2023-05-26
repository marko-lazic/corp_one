use bevy::log::info;
use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct Despawn;

#[derive(Component)]
pub struct Persistent;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
        app.add_system(teardown.in_schedule(OnExit(GameState::Loading)));
        app.add_system(teardown.in_schedule(OnExit(GameState::Playing)));
        app.add_system(teardown.in_schedule(OnExit(GameState::StarMap)));
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
