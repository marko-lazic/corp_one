use rand::{Rng, SeedableRng};
use wyrand::WyRand;

enum CalculationType {
    Linear,
    Sigmoid,
    Exponential,
    Capped { limit: f32 },
}

struct Chance {
    base: f32, // Base probability (e.g., 0.2 for 20%)
    increase: f32,
    calculation_type: CalculationType,
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
            calculation_type: CalculationType::Linear,
        }
    }
}

impl PseudoRandomDistribution {
    fn new(chance: Chance, seed: Option<u64>) -> Self {
        let current_chance = chance.base;
        Self {
            chance,
            current_chance,
            rng: WyRand::seed_from_u64(seed.unwrap_or_else(rand::random)),
        }
    }

    fn roll(&mut self) -> bool {
        let result = self.rng.gen::<f32>() < self.current_chance;

        if !result {
            // If the roll failed
            match self.chance.calculation_type {
                CalculationType::Linear => {
                    self.current_chance += self.chance.increase;
                }
                CalculationType::Sigmoid => {
                    let balance_factor = 2.0; // Adjust this value to control steepness
                    self.current_chance = (2.0 - 2.0 * self.chance.base)
                        * (1.0 / (1.0 + (-balance_factor * self.current_chance).exp()))
                        - (1.0 - 2.0 * self.chance.base);
                }
                CalculationType::Exponential => {
                    let base = 2.0; // You can change the base of the exponent
                    self.current_chance *= base;
                }
                CalculationType::Capped { limit } => {
                    self.current_chance = (self.current_chance + self.chance.increase).min(limit);
                }
            }

            // Ensure the chance doesn't exceed 100%
            self.current_chance = self.current_chance.min(1.0);
        } else {
            // If the roll succeeded
            self.current_chance = self.chance.base; // Reset the chance
        }

        return result; // Return whether the roll was successful
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
        assert_eq!(19, success);
        assert_eq!(81, failure);
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
        assert_eq!(41, success);
        assert_eq!(59, failure);
    }

    #[test]
    fn periodic_chance_random_seed() {
        let mut success = 0;
        let mut failure = 0;
        let mut prd = PseudoRandomDistribution::new(
            Chance {
                base: 0.2, // 20% base chance
                calculation_type: CalculationType::Exponential,
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
                calculation_type: CalculationType::Linear,
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
        assert_eq!(58, success);
        assert_eq!(42, failure);
    }
}
