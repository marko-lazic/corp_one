//! This module contains the shared code between the client and the server.

use bevy::utils::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use lightyear::{prelude::*, shared::config::Mode};

pub const FIXED_TIME_STEP_HZ: f64 = 64.0;

pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

/// The [`SharedConfig`] must be shared between the `ClientConfig` and `ServerConfig`
pub fn shared_config() -> SharedConfig {
    SharedConfig {
        // send an update every 100ms
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIME_STEP_HZ),
        },
        mode: Mode::Separate,
    }
}