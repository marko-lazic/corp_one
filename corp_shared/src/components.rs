pub const MAX_HEALTH: f64 = 100.0;
pub const MIN_HEALTH: f64 = 0.0;

#[derive(Default)]
pub struct Player {
    pub is_moving: bool,
}

pub struct Health {
    hit_points: f64,
}

impl Health {
    pub fn deal_damage(&mut self, damage: f64) {
        self.hit_points = (&self.hit_points - damage).max(MIN_HEALTH);
    }

    pub fn heal(&mut self, heal: f64) {
        self.hit_points = (&self.hit_points + heal).min(MAX_HEALTH);
    }

    pub fn get_hit_points(&self) -> &f64 {
        &self.hit_points
    }

    pub fn set_hit_points(&mut self, hit_points: f64) {
        self.hit_points = hit_points;
    }
}

impl Default for Health {
    fn default() -> Self {
        Health {
            hit_points: MAX_HEALTH,
        }
    }
}
