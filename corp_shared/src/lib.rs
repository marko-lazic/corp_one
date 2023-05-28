use std::net::{IpAddr, Ipv4Addr};

use bevy::math::Vec3;

pub mod items;
pub mod stats;
pub mod util;
pub mod world;

// connection
pub const SERVER_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const SERVER_PORT: u16 = 9001;

// world
pub const CLONING_SPAWN_POSITION: Vec3 = Vec3::from_array([0.; 3]);

pub mod prelude {
    pub use crate::items::inventory::*;
    pub use crate::items::item::*;
    pub use crate::stats::health::*;
    pub use crate::util::test_utils::*;
    pub use crate::world::faction::*;
    pub use crate::world::interactive::*;
    pub use crate::world::objects::backpack::*;
    pub use crate::world::objects::door::*;
    pub use crate::world::player::*;
}
