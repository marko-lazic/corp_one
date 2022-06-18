use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::NextState;

use corp_shared::prelude::{Health, CLONE_HEALTH_80};

use crate::asset::asset_loading::ColonyAssets;
use crate::constants::state::GameState;
use crate::world::colony::vortex::VortInEvent;
use crate::world::colony::Colony;
use crate::world::player::Player;
use crate::Game;

pub struct CloningPlugin;

impl CloningPlugin {
    fn check_if_dead_and_go_to_cloning(
        colony_assets: Res<ColonyAssets>,
        time: Res<Time>,
        mut game: ResMut<Game>,
        mut query: Query<&mut Health, With<Player>>,
        mut commands: Commands,
    ) {
        if let Some(mut health) = query.iter_mut().next() {
            if health.is_dead() {
                health.cloning_cooldown.tick(time.delta());
                if health.cloning_cooldown.finished() {
                    game.current_colony_asset = colony_assets.cloning.clone();
                    commands.insert_resource(NextState(GameState::LoadColony));
                }
            }
        }
    }

    fn vort_in_dead_player_to_cloning(
        mut game: ResMut<Game>,
        mut vortex_events: EventWriter<VortInEvent>,
    ) {
        if game.health.is_dead() {
            game.health.set_hit_points(CLONE_HEALTH_80);
            game.health.cloning_cooldown.reset();
            vortex_events.send(VortInEvent::vort(Colony::Cloning));
        }
    }
}

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::StarMap, Self::vort_in_dead_player_to_cloning);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::check_if_dead_and_go_to_cloning)
                .into(),
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use corp_shared::prelude::MIN_HEALTH;

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
            .with_system(kill_player.label(KILLING_LABEL))
            .with_system(CloningPlugin::check_if_dead_and_go_to_cloning.after(KILLING_LABEL));

        // Setup world
        let mut world = World::default();

        // Setup test entities
        let player_entity = world
            .spawn()
            .insert(Player::default())
            .insert(Health::default())
            .id();

        world.insert_resource(create_colony_assets());
        world.insert_resource(Time::default());
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
            world.resource::<State<GameState>>().current(),
            &GameState::Playing,
            "Game state changed to StarMap"
        );
    }

    #[test]
    fn test_vort_in_dead_player() {
        // Setup stage
        let mut stage = SystemStage::parallel()
            .with_system_set(State::<GameState>::get_driver())
            .with_system(CloningPlugin::vort_in_dead_player_to_cloning);

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
        world.init_resource::<Events<VortInEvent>>();
        world.insert_resource(create_colony_assets());
        world.insert_resource(State::new(GameState::Playing));

        // Run systems
        stage.run(&mut world);

        assert_eq!(
            world.resource::<Game>().health.get_health(),
            &CLONE_HEALTH_80,
            "Game component health is set to clone health"
        );

        assert_eq!(
            world.resource::<State<GameState>>().current(),
            &GameState::Playing,
            "Game state changed to Playing"
        );
    }

    fn create_colony_assets() -> ColonyAssets {
        ColonyAssets {
            iris: Default::default(),
            liberte: Default::default(),
            cloning: Default::default(),
        }
    }
}
