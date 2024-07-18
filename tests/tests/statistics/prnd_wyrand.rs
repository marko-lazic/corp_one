use rand::{Rng, SeedableRng};
use wyrand::WyRand;

struct Chance {
    base: f32, // Base probability (e.g., 0.2 for 20%)
    increase: f32,
}
struct PseudoRandomDistribution {
    chance: Chance,
    current_chance: f32,
    rng: WyRand, // Random number generator
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
    fn new(chance: Chance) -> Self {
        let current_chance = chance.base;
        let mut random_seed = [0u8; 8];
        getrandom::getrandom(&mut random_seed).expect("Unable to source entropy for seeding");
        Self {
            chance,
            current_chance,
            rng: WyRand::from_seed(random_seed),
        }
    }

    fn new_seed(chance: Chance, seed: [u8; 8]) -> Self {
        let current_chance = chance.base;
        Self {
            chance,
            current_chance,
            rng: WyRand::from_seed(seed),
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

    const SEED: [u8; 8] = [0; 8];

    #[test]
    fn initial_chance() {
        let mut success = 0;
        let mut failure = 0;
        for i in 0..100 {
            let mut prd = PseudoRandomDistribution::new_seed(
                Chance {
                    base: 0.2, // 20% base chance
                    ..Default::default()
                },
                [i; 8],
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
        assert_eq!(22, success);
        assert_eq!(78, failure);
    }

    #[test]
    fn periodic_chance() {
        let mut success = 0;
        let mut failure = 0;
        let mut prd = PseudoRandomDistribution::new_seed(
            Chance {
                base: 0.2, // 20% base chance
                ..Default::default()
            },
            SEED,
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
        assert_eq!(41, success);
        assert_eq!(59, failure);
    }

    #[test]
    fn periodic_chance_no_seed() {
        let mut success = 0;
        let mut failure = 0;
        let mut prd = PseudoRandomDistribution::new(Chance {
            base: 0.2, // 20% base chance
            ..Default::default()
        });
        for _ in 0..100 {
            if prd.roll() {
                success += 1;
            } else {
                failure += 1;
            }
        }
        println!("periodic_chance_no_seed");
        println!("Success: {success}");
        println!("Failure: {failure}");
    }

    #[test]
    fn increase_chance() {
        let mut success = 0;
        let mut failure = 0;
        let mut prd = PseudoRandomDistribution::new_seed(
            Chance {
                base: 0.2, // 20% base chance
                increase: 0.9,
            },
            SEED,
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
        assert_eq!(58, success);
        assert_eq!(42, failure);
    }
}
