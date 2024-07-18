use rand::{Rng, SeedableRng};
use wyrand::WyRand;

struct Ability {
    min_value: f32,
    max_value: f32,
}

impl Ability {
    fn new(min_value: f32, max_value: f32) -> Self {
        Self {
            min_value,
            max_value,
        }
    }

    fn calculate_value(&self, random_value: f32) -> f32 {
        self.min_value + (self.max_value - self.min_value) * random_value
    }

    fn calculate_inverse(&self, random_value: f32) -> f32 {
        self.max_value - (self.max_value - self.min_value) * random_value
    }
}

struct AbilityPairBuilder {
    ability_1: Ability,
    ability_2: Ability,
    rng: WyRand,
    inverse_correlation: bool,
    strong_correlation: bool,
    current_random_value: Option<f32>,
}

impl AbilityPairBuilder {
    fn new(ability_1: Ability, ability_2: Ability, rng: WyRand) -> Self {
        Self {
            ability_1,
            ability_2,
            rng,
            inverse_correlation: false,
            strong_correlation: false,
            current_random_value: None,
        }
    }

    fn inverse(mut self) -> Self {
        self.inverse_correlation = true;
        self
    }

    fn strong(mut self) -> Self {
        self.strong_correlation = true;
        self
    }

    fn random_value(&mut self) -> f32 {
        match self.current_random_value {
            None => {
                let random_value = self.rng.gen::<f32>();
                if self.strong_correlation {
                    self.current_random_value = Some(random_value);
                }
                random_value
            }
            Some(random_value) => random_value,
        }
    }

    fn build(&mut self) -> (f32, f32) {
        let random_value = self.random_value();
        let value_1 = self.ability_1.calculate_value(random_value);
        let value_2 = if self.inverse_correlation {
            let random_value = self.random_value();
            self.ability_2.calculate_inverse(random_value)
        } else {
            let random_value = self.random_value();
            self.ability_2.calculate_value(random_value)
        };
        self.current_random_value = None;
        (value_1, value_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_inverse_correlation() {
        let damage = Ability::new(90.0, 180.0);
        let stun = Ability::new(1.25, 2.2);

        let rng = WyRand::seed_from_u64(rand::random());
        let mut chaos_bolt = AbilityPairBuilder::new(damage, stun, rng)
            .inverse()
            .strong();

        for _ in 0..20 {
            let (damage_value, stun_value) = chaos_bolt.build();
            println!(
                "Inversely correlated damage and stun: {:.2}, {:.2}",
                damage_value, stun_value
            );
        }
    }

    #[test]
    fn calculate_positive_correlation() {
        let damage = Ability::new(90.0, 180.0);
        let stun = Ability::new(1.25, 2.2);

        let rng = WyRand::seed_from_u64(rand::random());
        let mut chaos_bolt = AbilityPairBuilder::new(damage, stun, rng);

        for _ in 0..20 {
            let (damage_value, stun_value) = chaos_bolt.build();
            println!(
                "Positively correlated damage and stun: {:.2}, {:.2}",
                damage_value, stun_value
            );
        }
    }
}
