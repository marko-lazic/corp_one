mod barrier;
mod vortex;

pub mod prelude {
    pub use super::{barrier::*, vortex::*};
    pub use crate::world::load_colony::*;
    pub use corp_shared::world::gameplay::area::*;
}
