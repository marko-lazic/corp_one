use aeronet_webtransport::wtransport::Identity;
use bevy::prelude::Resource;
use corp_shared::prelude::Colony;
use std::net::SocketAddr;

#[derive(Resource, Debug)]
pub struct ColonyAppConfig {
    pub colony: Colony,
    pub server_addr: SocketAddr,
    pub identity: Identity,
}

impl Clone for ColonyAppConfig {
    fn clone(&self) -> Self {
        Self {
            colony: self.colony.clone(),
            server_addr: self.server_addr.clone(),
            identity: self.identity.clone_identity(),
        }
    }
}
