use std::time::Duration;

use bevy::core::Timer;

pub const MAX_HEALTH: f32 = 100.0;
pub const CLONE_HEALTH_80: f32 = 80.0;
pub const MIN_HEALTH: f32 = 0.0;

#[derive(Clone, Debug, bevy::ecs::component::Component)]
pub struct Health {
    hit_points: f32,
    pub cloning_cooldown: Timer,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            cloning_cooldown: Timer::new(Duration::from_secs(1), false),
            hit_points: MAX_HEALTH,
        }
    }
}

impl Health {
    pub fn take_damage(&mut self, damage: f32) {
        self.hit_points = (&self.hit_points - damage).max(MIN_HEALTH);
    }

    pub fn heal(&mut self, heal: f32) {
        self.hit_points = (&self.hit_points + heal).min(MAX_HEALTH);
    }

    pub fn get_health(&self) -> &f32 {
        &self.hit_points
    }

    pub fn is_dead(&self) -> bool {
        &self.hit_points <= &MIN_HEALTH
    }

    pub fn set_hit_points(&mut self, hit_points: f32) {
        self.hit_points = hit_points;
    }

    pub fn kill_mut(&mut self) {
        self.hit_points = MIN_HEALTH;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_damage_hit_points_min_value() {
        let mut health = Health::default();
        health.take_damage(9000.);
        assert_eq!(health.get_health(), &MIN_HEALTH);
    }

    #[test]
    fn heal_hit_points_max_value() {
        let mut health = Health::default();
        health.heal(200.);
        assert_eq!(health.get_health(), &MAX_HEALTH);
    }

    #[test]
    fn take_damage_and_heal() {
        let mut health = Health::default();
        health.take_damage(50.);
        health.heal(30.);
        assert_eq!(health.get_health(), &80.);
    }
}
