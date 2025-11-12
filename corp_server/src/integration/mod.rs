use std::time::Duration;

mod token;

pub use token::*;
pub const RETRY_DELAY: Duration = Duration::from_millis(500);
