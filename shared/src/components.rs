#[derive(Default)]
pub struct Player {
    pub is_moving: bool,
}

pub struct Health {
    hit_points: u32,
}

impl Health {
    pub fn deal_damage(&mut self, damage: u32) {
        self.hit_points -= damage;
    }

    pub fn get_hit_points(&self) -> u32 {
        self.hit_points
    }

    pub fn set_hit_points(&mut self, hit_points: u32) {
        self.hit_points = hit_points;
    }
}

impl Default for Health {
    fn default() -> Self {
        Health { hit_points: 100 }
    }
}
