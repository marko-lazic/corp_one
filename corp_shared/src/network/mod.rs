pub mod constants {
    pub const TICK_RATE: u16 = 30;
}

mod auth;
mod replicate_rules;
mod user;

pub use auth::*;
pub use constants::*;
pub use replicate_rules::*;
pub use user::*;
