use crate::{GameProxy, ProxyConfig};
use corp_shared::prelude::Colony;
use log::info;
use std::collections::HashMap;

pub async fn init() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut routes: HashMap<Colony, String> = HashMap::new();

    let iris_addr = "https://localhost:25565";
    let cloning_addr = "https://localhost:25566";
    let starmap_addr = "https://localhost:25567";
    let liberte_addr = "https://localhost:25568";

    routes.insert(Colony::Iris, iris_addr.into());
    routes.insert(Colony::Cloning, cloning_addr.into());
    routes.insert(Colony::StarMap, starmap_addr.into());
    routes.insert(Colony::Liberte, liberte_addr.into());

    let config = ProxyConfig {
        port: 25560,
        routes,
        cert_path: None,
        key_path: None,
    };

    info!("Proxy Routes:");
    for (id, addr) in &config.routes {
        info!("  {} -> {}", id, addr);
    }

    let proxy = GameProxy::new(config);
    proxy.run().await?;
    Ok(())
}
