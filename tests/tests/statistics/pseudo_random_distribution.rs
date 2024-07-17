use rand::Rng;
use wyrand::WyRand;

struct PseudoRandomDistribution {
    base_chance: f32, // Base probability (e.g., 0.2 for 20%)
    current_chance: f32,
    rng: WyRand, // Random number generator
}

impl PseudoRandomDistribution {
    fn new(base_chance: f32, seed: u64) -> Self {
        Self {
            base_chance,
            current_chance: base_chance,
            rng: WyRand::new(seed),
        }
    }

    fn roll(&mut self) -> bool {
        let result = self.rng.gen::<f32>() < self.current_chance;
        if result {
            // Success: Reset chance
            self.current_chance = self.base_chance;
            true
        } else {
            // Failure: Increase chance
            // You'll need to decide on the specific increment logic here
            self.current_chance += 0.1; // Example: Increase by 10%
            false
        }
    }
}

const SEED: u64 = 12345689;

#[test]
fn initial_chance() {
    let mut success = 0;
    let mut failure = 0;
    for i in 0..100 {
        let mut prd = PseudoRandomDistribution::new(0.2, SEED + i); // 20% base chance
        if prd.roll() {
            success += 1;
        } else {
            failure += 1;
        }
    }
    println!("Success: {success}");
    println!("Failure: {failure}");
    assert_eq!(16, success);
    assert_eq!(84, failure);
}

#[test]
fn periodic_chance() {
    let mut success = 0;
    let mut failure = 0;
    let mut prd = PseudoRandomDistribution::new(0.2, SEED); // 20% base chance
    for _ in 0..100 {
        if prd.roll() {
            success += 1;
        } else {
            failure += 1;
        }
    }
    println!("Success: {success}");
    println!("Failure: {failure}");
    assert_eq!(38, success);
    assert_eq!(62, failure);
}
