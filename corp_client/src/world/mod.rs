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
    pub use super::{
        animator::*, ccc::*, cloning::*, colony::prelude::*, physics::*, player::*, shader::*,
        star_map::*, world::*,
    };
}
