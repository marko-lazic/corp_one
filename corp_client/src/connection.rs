use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};

use corp_shared::{SERVER_HOST, SERVER_PORT};

use crate::constants::state::GameState;

pub struct ConnectionPlugin;

impl ConnectionPlugin {
    fn startup(mut net: ResMut<NetworkResource>) {
        let server_address = SocketAddr::new(SERVER_HOST, SERVER_PORT);
        info!("Connecting to {}...", server_address);
        net.connect(server_address);
    }

    fn send_pings(mut net: ResMut<NetworkResource>, time: Res<Time>) {
        if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
            net.broadcast(Packet::from("PING"));
        }
    }

    fn handle_packets(mut reader: EventReader<NetworkEvent>) {
        for event in reader.iter() {
            match event {
                NetworkEvent::Packet(handle, packet) => {
                    let message = String::from_utf8_lossy(packet);
                    info!("Got packet on [{}]: {}", handle, message);
                }
                _ => {}
            }
        }
    }
}

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NetworkingPlugin::default());
        app.add_startup_system(Self::startup.system());
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(Self::send_pings.system())
                .with_system(Self::handle_packets.system()),
        );
    }
}
