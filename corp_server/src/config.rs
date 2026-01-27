use aeronet_webtransport::wtransport::Identity;
use bevy::prelude::Resource;
use corp_shared::prelude::Colony;
use std::net::SocketAddr;

#[derive(Resource, Debug)]
pub struct GameServerConfig {
    pub colony: Colony,
    pub server_addr: SocketAddr,
    pub identity: Identity,
}

impl Clone for GameServerConfig {
    fn clone(&self) -> Self {
        Self {
            colony: self.colony,
            server_addr: self.server_addr,
            identity: self.identity.clone_identity(),
        }
    }
}
