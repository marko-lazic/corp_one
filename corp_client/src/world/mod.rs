mod animator;
mod ccc;
mod cloning;
mod colony;
mod physics;
mod player;
mod shader;
mod star_map;
mod world;

pub mod prelude {
    pub use super::{ccc::*, physics::*, player::*, world::*};
}
