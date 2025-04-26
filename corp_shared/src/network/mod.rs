pub mod constants {
    pub const TICK_RATE: u16 = 30;
}

pub mod replicate_rules;

pub mod prelude {
    pub use super::{constants::*, replicate_rules::*};
}
