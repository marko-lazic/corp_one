use crate::server::Tokens;
use aeronet_webtransport::wtransport::Identity;
use bevy::prelude::Resource;
use corp_shared::prelude::Colony;
use kameo::actor::ActorRef;
use std::net::SocketAddr;

#[derive(Resource, Debug)]
pub struct GameServerConfig {
    pub colony: Colony,
    pub server_addr: SocketAddr,
    pub identity: Identity,
    pub tokens_ref: ActorRef<Tokens>,
}

impl Clone for GameServerConfig {
    fn clone(&self) -> Self {
        Self {
            colony: self.colony.clone(),
            server_addr: self.server_addr.clone(),
            identity: self.identity.clone_identity(),
            tokens_ref: self.tokens_ref.clone(),
        }
    }
}
