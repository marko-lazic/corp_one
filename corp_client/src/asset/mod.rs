pub use colony_configs::*;

mod asset_loading;
mod colony_configs;
mod path;

pub mod prelude {
    pub use super::{asset_loading::*, colony_configs::*, path::ASSET_PATH};
}
