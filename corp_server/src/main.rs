use bevy::app::ScheduleRunnerSettings;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};
use corp_shared::{SERVER_HOST, SERVER_PORT};
use std::{net::SocketAddr, time::Duration};

fn main() {
    let frames_per_second = Duration::from_secs_f64(1.0 / 60.0);
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(frames_per_second))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(startup.system())
        .add_system(handle_packets.system())
        .run();
}

fn startup(mut net: ResMut<NetworkResource>) {
    let server_address = SocketAddr::new(SERVER_HOST, SERVER_PORT);
    info!("Starting server on {}", server_address);
    net.listen(server_address, None, None);
}

fn handle_packets(
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
