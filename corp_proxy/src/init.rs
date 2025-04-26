use crate::{GameProxy, ProxyConfig};
use log::info;
use std::collections::HashMap;

pub async fn init() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let mut routes: HashMap<String, String> = HashMap::new();

    let iris_addr = "https://localhost:25565";
    let cloning_addr = "https://localhost:25566";
    let starmap_addr = "https://localhost:25567";

    routes.insert("default".into(), iris_addr.into());
    routes.insert("iris".into(), iris_addr.into());
    routes.insert("cloning".into(), cloning_addr.into());
    routes.insert("starmap".into(), starmap_addr.into());

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
