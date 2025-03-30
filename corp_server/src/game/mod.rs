mod app;
mod config;
mod database;
mod instance;
mod server;

pub mod prelude {
    pub use super::{app::*, config::*, instance::*};
}
