use aeronet_webtransport::{
    cert, wtransport,
    wtransport::{endpoint::ConnectOptions, tls::Sha256Digest},
};
use bevy::{log::warn, prelude::Resource};
use corp_shared::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct ClientSettings {
    host: String,
    cert_hash: String,
    keep_alive_interval: Duration,
    max_idle_timeout: Duration,
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            host: "https://[::1]:25560".to_string(),
            cert_hash: "".to_string(),
            keep_alive_interval: Duration::from_secs(1),
            max_idle_timeout: Duration::from_secs(5),
        }
    }
}

impl ClientSettings {
    pub fn target(&self, route: &Colony, token: &AuthToken) -> ConnectOptions {
        ConnectOptions::builder(&self.host)
            .add_header("x-route", route)
            .add_header("x-token", token)
            .build()
    }
    pub fn client_config(&self) -> wtransport::ClientConfig {
        let config = wtransport::ClientConfig::builder().with_bind_default();

        let config = if self.cert_hash.is_empty() {
            warn!("Connecting without certificate validation");
            config.with_no_cert_validation()
        } else {
            match cert::hash_from_b64(&self.cert_hash) {
                Ok(hash) => config.with_server_certificate_hashes([Sha256Digest::new(hash)]),
                Err(err) => {
                    warn!("Failed to read certificate hash from string: {err:?}");
                    config.with_server_certificate_hashes([])
                }
            }
        };

        config
            .keep_alive_interval(Some(self.keep_alive_interval))
            .max_idle_timeout(Some(self.max_idle_timeout))
            .expect("should be a valid idle timeout")
            .build()
    }
}
