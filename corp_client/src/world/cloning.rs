use bevy::core::FixedTimestep;
use bevy::prelude::*;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::Game;

pub struct CloningPlugin;

impl CloningPlugin {
    fn check_player_died(
        mut game: ResMut<Game>,
        mut game_state: ResMut<State<GameState>>,
        healths: Query<&Health, With<Player>>,
    ) {
        if let Some(health) = healths.iter().next() {
            if health.is_dead() {
                game.health = health.clone();
                let _result = game_state.set(GameState::StarMap);
            }
        }
    }
}

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::check_player_died.system()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kill_player(mut healths: Query<&mut Health, With<Player>>) {
        for mut health in healths.iter_mut() {
            health.kill();
        }
    }

    const KILLING_LABEL: &'static str = "killing";

    #[test]
    fn state_changes_to_starmap_when_player_is_dead() {
        // Setup stage
        let mut stage = SystemStage::parallel()
            .with_system_set(State::<GameState>::get_driver())
            .with_system(kill_player.system().label(KILLING_LABEL))
            .with_system(
                CloningPlugin::check_player_died
                    .system()
                    .after(KILLING_LABEL),
            );

        // Setup world
        let mut world = World::default();

        // Setup test entities
        let player_entity = world
            .spawn()
            .insert(Player::default())
            .insert(Health::default())
            .id();

        world.insert_resource(Game::default());
        world.insert_resource(State::new(GameState::Playing));

        // Run systems
        stage.run(&mut world);

        // Check resulting changes
        assert!(world.get::<Player>(player_entity).is_some());

        let dead_player_expected_health: f64 = 0.0;
        assert_eq!(
            *world.get::<Health>(player_entity).unwrap().get_health(),
            dead_player_expected_health,
            "Player is dead"
        );

        assert_eq!(
            world.get_resource::<State<GameState>>().unwrap().current(),
            &GameState::StarMap,
            "Game state changed to StarMap"
        );
    }
}
