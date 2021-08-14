pub mod components;

use std::net::{IpAddr, Ipv4Addr};

use bevy::math::{const_vec3, Vec3};

// connection
pub const SERVER_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const SERVER_PORT: u16 = 9001;

// world
pub const CLONING_SPAWN_POSITION: Vec3 = const_vec3!([0.; 3]);
