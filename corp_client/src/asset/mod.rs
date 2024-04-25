pub use asset_loading::*;
pub use colony_configs::*;

mod asset_loading;
mod colony_configs;

pub mod prelude {
    pub use super::{asset_loading::ColonyConfigAssets, colony_configs::Colony};
}
