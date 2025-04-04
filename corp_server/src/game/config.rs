use bevy::prelude::Resource;
use corp_shared::prelude::Colony;
use std::net::SocketAddr;

#[derive(Resource, Debug, Clone, Copy)]
pub struct GameInstanceConfig {
    pub colony: Colony,
    pub server_addr: SocketAddr,
}
