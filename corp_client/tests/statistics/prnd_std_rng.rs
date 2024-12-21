use rand::{prelude::StdRng, Rng, SeedableRng};

struct Chance {
    base: f32, // Base probability (e.g., 0.2 for 20%)
    increase: f32,
}
struct PseudoRandomDistribution {
    chance: Chance,
    current_chance: f32,
    rng: StdRng, // Random number generator
}

impl Default for Chance {
    fn default() -> Self {
        Self {
            base: 0.1,
            increase: 0.1,
        }
    }
}

impl PseudoRandomDistribution {
    fn new(chance: Chance, seed: Option<u64>) -> Self {
        let current_chance = chance.base;
        Self {
            chance,
            current_chance,
            rng: StdRng::seed_from_u64(seed.unwrap_or_else(rand::random)),
        }
    }

    fn roll(&mut self) -> bool {
        let result = self.rng.gen::<f32>() < self.current_chance;
        if result {
            // Success: Reset chance
            self.current_chance = self.chance.base;
            true
        } else {
            // Failure: Increase chance
            // You'll need to decide on the specific increment logic here
            self.current_chance += self.chance.increase; // Example: Increase by 0.1 for 10%
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 0;

    #[test]
    fn initial_chance() {
        let mut success = 0;
        let mut failure = 0;
        for seed in 0..100 {
            let mut prd = PseudoRandomDistribution::new(
                Chance {
                    base: 0.2, // 20% base chance
                    ..Default::default()
                },
                Some(seed),
            );
            if prd.roll() {
                success += 1;
            } else {
                failure += 1;
            }
        }

        println!("initial_chance");
        println!("Success: {success}");
        println!("Failure: {failure}");
        assert_eq!(14, success);
        assert_eq!(86, failure);
    }

    #[test]
    fn periodic_chance() {
        let mut success = 0;
        let mut failure = 0;
        let mut prd = PseudoRandomDistribution::new(
            Chance {
                base: 0.2, // 20% base chance
                ..Default::default()
            },
            Some(SEED),
        );
        for _ in 0..100 {
            if prd.roll() {
                success += 1;
            } else {
                failure += 1;
            }
        }
        println!("periodic_chance");
        println!("Success: {success}");
        println!("Failure: {failure}");
        assert_eq!(31, success);
        assert_eq!(69, failure);
    }

    #[test]
    fn periodic_chance_random_seed() {
        let mut success = 0;
        let mut failure = 0;
        let mut prd = PseudoRandomDistribution::new(
            Chance {
                base: 0.2, // 20% base chance
                ..Default::default()
            },
            None,
        );
        for _ in 0..100 {
            if prd.roll() {
                success += 1;
            } else {
                failure += 1;
            }
        }
        println!("periodic_chance_random_seed");
        println!("Success: {success}");
        println!("Failure: {failure}");
    }

    #[test]
    fn increase_chance() {
        let mut success = 0;
        let mut failure = 0;
        let mut prd = PseudoRandomDistribution::new(
            Chance {
                base: 0.2, // 20% base chance
                increase: 0.9,
            },
            Some(SEED),
        );
        for _ in 0..100 {
            if prd.roll() {
                success += 1;
            } else {
                failure += 1;
            }
        }

        println!("increase_chance");
        println!("Success: {success}");
        println!("Failure: {failure}");
        assert_eq!(56, success);
        assert_eq!(44, failure);
    }
}
