use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};

use corp_shared::{SERVER_HOST, SERVER_PORT};

pub struct ServerPlugin;

impl ServerPlugin {
    fn startup(mut net: ResMut<NetworkResource>) {
        let server_address = SocketAddr::new(SERVER_HOST, SERVER_PORT);
        info!("Starting server on {}", server_address);
        net.listen(server_address, None, None);
    }

    fn send_pongs(
        mut net: ResMut<NetworkResource>,
        time: Res<Time>,
        mut reader: EventReader<NetworkEvent>,
    ) {
        for event in reader.iter() {
            match event {
                NetworkEvent::Packet(handle, packet) => {
                    let message = String::from_utf8_lossy(packet);
                    info!("Got packet on [{}]: {}", handle, message);
                    if message == "PING" {
                        let message = format!("PONG @ {}", time.seconds_since_startup());
                        match net.send(*handle, Packet::from(message)) {
                            Ok(()) => {
                                info!("Sent PONG");
                            }
                            Err(error) => {
                                info!("PONG send error: {}", error);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(NetworkingPlugin::default());
        app.add_startup_system(Self::startup.system());
        app.add_system(Self::send_pongs.system());
    }
}
