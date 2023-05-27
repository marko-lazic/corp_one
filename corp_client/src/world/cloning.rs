use bevy::prelude::*;

use corp_shared::prelude::*;
use corp_shared::prelude::{Health, CLONE_HEALTH_80};

use crate::asset::asset_loading::ColonyAssets;
use crate::state::GameState;
use crate::world::colony::vortex::VortInEvent;
use crate::world::colony::Colony;
use crate::Game;

pub struct CloningPlugin;

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            Self::vort_in_dead_player_to_cloning.in_schedule(OnEnter(GameState::StarMap)),
        );
        app.add_system(Self::check_if_dead_and_go_to_cloning.in_set(OnUpdate(GameState::Playing)));
    }
}

impl CloningPlugin {
    fn check_if_dead_and_go_to_cloning(
        colony_assets: Res<ColonyAssets>,
        time: Res<Time>,
        mut game: ResMut<Game>,
        mut query: Query<&mut Health, With<Player>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        if let Some(mut health) = query.iter_mut().next() {
            if health.is_dead() {
                health.cloning_cooldown.tick(time.delta());
                if health.cloning_cooldown.finished() {
                    game.current_colony_asset = colony_assets.cloning.clone();
                    next_state.set(GameState::LoadColony);
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

    #[test]
    fn test_vort_out_dead_player() {
        // given
        let mut app = App::new();
        init_time(&mut app);
        app.add_state::<GameState>();
        app.add_systems((kill_player, CloningPlugin::check_if_dead_and_go_to_cloning).chain());
        app.insert_resource(create_colony_assets());
        app.insert_resource(Game::default());
        let player_entity = app.world.spawn((Player::default(), Health::default())).id();

        // when
        app.update();

        // Check resulting changes
        assert!(app.world.get::<Player>(player_entity).is_some());

        assert_eq!(
            *app.world.get::<Health>(player_entity).unwrap().get_health(),
            MIN_HEALTH.clone(),
            "Player is dead"
        );
    }

    #[test]
    fn test_vort_in_dead_player() {
        // Setup stage
        let mut schedule = Schedule::default();
        schedule.add_system(CloningPlugin::vort_in_dead_player_to_cloning);

        // Setup world
        let mut world = World::default();
        // Setup test entities
        let _player_entity = world.spawn((Player::default(), Health::default())).id();

        let mut game = Game::default();
        game.health.kill_mut();
        world.insert_resource(game);
        world.init_resource::<Events<VortInEvent>>();
        world.insert_resource(create_colony_assets());

        // Run systems
        schedule.run(&mut world);

        assert_eq!(
            world.resource::<Game>().health.get_health(),
            &CLONE_HEALTH_80,
            "Game component health is set to clone health"
        );
    }

    fn init_time(app: &mut App) {
        app.init_resource::<Time>();
        let mut time = Time::default();
        time.update();
        app.world.insert_resource(time);
    }

    fn create_colony_assets() -> ColonyAssets {
        ColonyAssets {
            iris: Default::default(),
            liberte: Default::default(),
            cloning: Default::default(),
        }
    }
}
