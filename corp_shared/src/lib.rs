use std::net::{IpAddr, Ipv4Addr};

use bevy::math::Vec3;

pub mod asset;
mod log;
pub mod network;
mod state;
pub mod util;
pub mod world;

// connection
pub const SERVER_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const SERVER_PORT: u16 = 9001;

// world
pub const CLONING_SPAWN_POSITION: Vec3 = Vec3::from_array([0.; 3]);

pub mod prelude {
    pub use crate::{asset::*, log::*, network::*, state::*, util::*, world::*};
}
