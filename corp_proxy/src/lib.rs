pub mod init;

use log::error;

use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tracing::{info, info_span, Instrument};
use wtransport::{
    endpoint::{IncomingSession, SessionRequest}, ClientConfig, Connection, Endpoint, Identity,
    ServerConfig,
};

/// Configuration for the Game Proxy
#[derive(Clone, Debug)]
pub struct ProxyConfig {
    /// The port of the proxy server to listen on
    pub port: u16,
    /// Routes map (world identifier -> backend server address)
    pub routes: HashMap<String, String>,
    /// TLS certificate path for WebTransport
    pub cert_path: Option<PathBuf>,
    /// TLS key path for WebTransport
    pub key_path: Option<PathBuf>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            routes: HashMap::new(),
            cert_path: None,
            key_path: None,
        }
    }
}

type Routes = Arc<RwLock<HashMap<String, String>>>;

/// A game proxy that routes WebTransport traffic
pub struct GameProxy {
    config: ProxyConfig,
}

impl GameProxy {
    pub fn new(config: ProxyConfig) -> Self {
        Self { config }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let cert_pemfile = self.config.cert_path.unwrap_or("./certs/server.pem".into());
        let private_key_pemfile = self.config.key_path.unwrap_or("./certs/server.key".into());
        let identity = Identity::load_pemfiles(cert_pemfile, private_key_pemfile).await?;
        let config = ServerConfig::builder()
            .with_bind_default(self.config.port)
            .with_identity(identity)
            .keep_alive_interval(Some(Duration::from_secs(1)))
            .max_idle_timeout(Some(Duration::from_secs(5)))
            .expect("should be a valid idle timeout")
            .build();
        let server = Endpoint::server(config)?;
        info!("Proxy listening on 127.0.0.1:{}", self.config.port);

        let routes = Arc::new(RwLock::new(self.config.routes));
        for id in 0.. {
            let incoming_session = server.accept().await;
            info!("Incoming connection: {}", id);
            tokio::spawn(
                handle_connection(incoming_session, routes.clone())
                    .instrument(info_span!("Connection", id)),
            );
        }

        Ok(())
    }
}

async fn handle_connection(incoming_session: IncomingSession, routes: Routes) {
    let result = Dispatch::new(routes).run(incoming_session).await;
    error!("{:?}", result);
}

use wtransport::endpoint::ConnectOptions;

struct Dispatch {
    routes: Routes,
}

impl Dispatch {
    pub fn new(routes: Routes) -> Self {
        Self { routes }
    }

    pub async fn run(&self, session: IncomingSession) -> anyhow::Result<()> {
        let req = session.await?;

        let route = self.get_route(&req)?;
        let backend_addr = self
            .get_backend(&route)
            .await
            .ok_or(anyhow::anyhow!("No backend found for route {}", route))?;

        // Connect to backend
        let config = ClientConfig::builder()
            .with_bind_default()
            .with_no_cert_validation()
            .build();
        let connect_options = ConnectOptions::builder(backend_addr).build();
        let backend_client = Endpoint::client(config)?.connect(connect_options).await?;
        let frontend_client = req.accept().await?;

        let mut tasks: Vec<futures::future::BoxFuture<'static, anyhow::Result<()>>> = Vec::new();

        // Proxy bidirectional streams
        tasks.push(Box::pin(Self::proxy_stream(
            frontend_client.clone(),
            backend_client.clone(),
        )) as _);

        // Proxy unidirectional streams
        tasks.push(Box::pin(Self::proxy_client_to_backend(
            frontend_client.clone(),
            backend_client.clone(),
        )) as _);

        tasks.push(Box::pin(Self::proxy_backend_to_client(
            backend_client.clone(),
            frontend_client.clone(),
        )) as _);

        // Proxy datagrams
        tasks.push(Box::pin(Self::proxy_datagrams(
            frontend_client.clone(),
            backend_client.clone(),
        )) as _);

        // Wait for all tasks to complete
        futures::future::join_all(tasks).await;

        Ok(())
    }

    async fn proxy_stream(frontend: Connection, backend: Connection) -> anyhow::Result<()> {
        loop {
            match frontend.accept_bi().await {
                Ok((mut to_client, mut from_client)) => {
                    match backend.open_bi().await?.await {
                        Ok((mut to_backend, mut from_backend)) => {
                            // spawn one task per stream-pair
                            tokio::spawn(async move {
                                // client → backend
                                let _ = tokio::io::copy(&mut from_client, &mut to_backend).await;
                                let _ = to_backend.finish().await;

                                // backend → client
                                let _ = tokio::io::copy(&mut from_backend, &mut to_client).await;
                                let _ = to_client.finish().await;
                            });
                        }
                        Err(e) => {
                            error!("proxy_stream: failed to open bidi to backend: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    // when session closes, break; otherwise log and keep looping
                    if e.to_string().contains("closed") {
                        break;
                    }
                    error!("proxy_stream: failed to accept bidi from client: {:?}", e);
                }
            }
        }
        Ok(())
    }

    async fn proxy_client_to_backend(
        frontend: Connection,
        backend: Connection,
    ) -> anyhow::Result<()> {
        loop {
            match frontend.accept_uni().await {
                Ok(mut from_client) => {
                    match backend.open_uni().await?.await {
                        Ok(mut to_backend) => {
                            // fire-and-forget the copy
                            tokio::spawn(async move {
                                let _ = tokio::io::copy(&mut from_client, &mut to_backend).await;
                                let _ = to_backend.finish().await;
                            });
                        }
                        Err(e) => {
                            error!("failed to open uni to backend: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    // break on closed session, otherwise log and continue
                    if e.to_string().contains("closed") {
                        break;
                    }
                    error!("failed to accept uni from client: {:?}", e);
                }
            }
        }
        Ok(())
    }

    /// Forward every uni‐stream from `backend` → `frontend` in a loop.
    async fn proxy_backend_to_client(
        backend: Connection,
        frontend: Connection,
    ) -> anyhow::Result<()> {
        loop {
            match backend.accept_uni().await {
                Ok(mut from_backend) => match frontend.open_uni().await?.await {
                    Ok(mut to_client) => {
                        tokio::spawn(async move {
                            let _ = tokio::io::copy(&mut from_backend, &mut to_client).await;
                            let _ = to_client.finish().await;
                        });
                    }
                    Err(e) => {
                        error!("failed to open uni to client: {:?}", e);
                    }
                },
                Err(e) => {
                    if e.to_string().contains("closed") {
                        break;
                    }
                    error!("failed to accept uni from backend: {:?}", e);
                }
            }
        }
        Ok(())
    }

    async fn proxy_datagrams(
        frontend_connection: Connection,
        backend_connection: Connection,
    ) -> anyhow::Result<()> {
        // client → backend
        let frontend = frontend_connection.clone();
        let backend = backend_connection.clone();
        let c2b = tokio::spawn(async move {
            while let Ok(datagram) = frontend.receive_datagram().await {
                let _ = backend.send_datagram(datagram.payload());
            }
        });

        // backend → client
        let frontend = frontend_connection.clone();
        let backend = backend_connection.clone();
        let b2c = tokio::spawn(async move {
            while let Ok(datagram) = backend.receive_datagram().await {
                let _ = frontend.send_datagram(datagram.payload());
            }
        });

        let _ = tokio::try_join!(c2b, b2c)?;
        Ok(())
    }

    async fn get_backend(&self, world_id: &str) -> Option<String> {
        let routes = self.routes.read().await;
        routes.get(world_id).cloned()
    }

    // Extract the world identifier from various request parts
    fn get_route(&self, req: &SessionRequest) -> anyhow::Result<String> {
        let route = req
            .headers()
            .get("x-route")
            .ok_or(anyhow::anyhow!("No route header"))?;
        Ok(route.clone())
    }
}
