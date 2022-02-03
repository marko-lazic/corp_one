use bevy::core::FixedTimestep;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::colony::vortex::{VortInEvent, VortOutEvent};
use crate::world::colony::Colony;
use crate::Game;

pub struct CloningPlugin;

impl CloningPlugin {
    fn run_if_dead(query: Query<&Health, (With<Player>, Changed<Health>)>) -> ShouldRun {
        if let Some(health) = query.iter().next() {
            if health.is_dead() {
                return ShouldRun::Yes;
            }
        }
        ShouldRun::No
    }

    fn vort_out_dead_player_to_starmap(mut vort_out_events: EventWriter<VortOutEvent>) {
        info!("Vorting out dead player to starmap!");
        vort_out_events.send(VortOutEvent);
    }

    fn vort_in_dead_player_to_cloning(
        mut game: ResMut<Game>,
        mut vortex_events: EventWriter<VortInEvent>,
    ) {
        if game.health.is_dead() {
            game.health.set_hit_points(CLONE_HEALTH_80);
            vortex_events.send(VortInEvent::vort(Colony::Cloning));
        }
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum CloningSystem {
    VortOut,
}

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::StarMap)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::vort_in_dead_player_to_cloning.system())
                .after(CloningSystem::VortOut),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(Self::run_if_dead)
                .with_system(Self::vort_out_dead_player_to_starmap),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kill_player(mut healths: Query<&mut Health, With<Player>>) {
        for mut health in healths.iter_mut() {
            health.kill_mut();
        }
    }

    const KILLING_LABEL: &'static str = "killing";

    #[test]
    fn test_vort_out_dead_player() {
        // Setup stage
        let mut stage = SystemStage::parallel()
            .with_system_set(State::<GameState>::get_driver())
            .with_system(kill_player.system().label(KILLING_LABEL))
            .with_system(
                CloningPlugin::vort_out_dead_player_to_starmap
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

        assert_eq!(
            *world.get::<Health>(player_entity).unwrap().get_health(),
            MIN_HEALTH.clone(),
            "Player is dead"
        );

        assert_eq!(
            world.get_resource::<State<GameState>>().unwrap().current(),
            &GameState::StarMap,
            "Game state changed to StarMap"
        );
    }

    #[test]
    fn test_vort_in_dead_player() {
        // Setup stage
        let mut stage = SystemStage::parallel()
            .with_system_set(State::<GameState>::get_driver())
            .with_system(CloningPlugin::vort_in_dead_player_to_cloning.system());

        // Setup world
        let mut world = World::default();
        // Setup test entities
        let _player_entity = world
            .spawn()
            .insert(Player::default())
            .insert(Health::default())
            .id();

        let mut game = Game::default();
        game.health.kill_mut();
        world.insert_resource(game);
        world.insert_resource(State::new(GameState::StarMap));

        // Run systems
        stage.run(&mut world);

        assert_eq!(
            world.get_resource::<Game>().unwrap().health.get_health(),
            &CLONE_HEALTH_80,
            "Game component health is set to clone health"
        );

        assert_eq!(
            world.get_resource::<State<GameState>>().unwrap().current(),
            &GameState::Playing,
            "Game state changed to Playing"
        );
    }
}
