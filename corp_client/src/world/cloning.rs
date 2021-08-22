use bevy::core::FixedTimestep;
use bevy::prelude::*;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::constants::tick;

pub struct CloningPlugin;

impl CloningPlugin {
    fn respawn_dead_player(mut query: Query<(&mut Transform, &mut Health), With<Player>>) {
        for (mut transform, mut health) in query.iter_mut() {
            if health.get_hit_points() == &MIN_HEALTH {
                transform.translation = CLONING_SPAWN_POSITION;
                health.set_hit_points(MAX_HEALTH);
            }
        }
    }
}

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::respawn_dead_player.system()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn move_player_system(mut query: Query<&mut Transform, With<Player>>) {
        for mut transform in query.iter_mut() {
            transform.translation = Vec3::new(42., 42., 42.);
        }
    }

    fn kill_player_system(mut query: Query<&mut Health, With<Player>>) {
        for mut health in query.iter_mut() {
            health.kill();
        }
    }

    #[test]
    fn did_respawn() {
        // Setup world
        let mut world = World::default();

        // Setup stage with our two systems
        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(move_player_system.system().before("killing"));
        update_stage.add_system(
            kill_player_system
                .system()
                .label("killing")
                .before("cloning"),
        );
        update_stage.add_system(CloningPlugin::respawn_dead_player.system().label("cloning"));

        // Setup test entities
        let player_id = world
            .spawn()
            .insert(Player::default())
            .insert(Transform::default())
            .insert(Health::default())
            .id();

        // Run systems
        update_stage.run(&mut world);

        // Check resulting changes
        assert!(world.get::<Player>(player_id).is_some());

        let expected_hit_points = 100;
        assert_eq!(
            world.get::<Health>(player_id).unwrap().get_hit_points(),
            expected_hit_points
        );

        let expected_translation: Vec3 = Vec3::new(0., 0., 0.);
        assert_eq!(
            world.get::<Transform>(player_id).unwrap().translation,
            expected_translation
        )
    }
}
