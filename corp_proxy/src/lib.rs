pub mod init;

use log::error;

use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::Arc, time::Duration};
use tokio::sync::{mpsc, RwLock};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, trace, trace_span, warn, Instrument};
use wtransport::{
    endpoint::IncomingSession, ClientConfig, Connection, Endpoint, Identity, ServerConfig, VarInt,
};

/// Configuration for the Game Proxy
#[derive(Clone, Debug)]
pub struct ProxyConfig {
    /// The port of the proxy server to listen on
    pub port: u16,
    /// Routes map (world identifier -> backend server address)
    pub routes: HashMap<Colony, String>,
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

type Routes = Arc<RwLock<HashMap<Colony, String>>>;

/// Connection state tracking for proper lifecycle management
#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connected,
    Disconnecting,
    Disconnected,
}

/// Connection manager that tracks connection state and handles cleanup
#[derive(Clone)]
pub struct ConnectionManager {
    state: Arc<RwLock<ConnectionState>>,
    cancellation_token: CancellationToken,
    cleanup_tx: mpsc::UnboundedSender<()>,
}

impl ConnectionManager {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<()>) {
        let (cleanup_tx, cleanup_rx) = mpsc::unbounded_channel();
        (
            Self {
                state: Arc::new(RwLock::new(ConnectionState::Connected)),
                cancellation_token: CancellationToken::new(),
                cleanup_tx,
            },
            cleanup_rx,
        )
    }

    pub async fn is_connected(&self) -> bool {
        matches!(*self.state.read().await, ConnectionState::Connected)
    }

    pub async fn mark_disconnecting(&self) {
        *self.state.write().await = ConnectionState::Disconnecting;
        self.cancellation_token.cancel();
    }

    pub async fn mark_disconnected(&self) {
        *self.state.write().await = ConnectionState::Disconnected;
        let _ = self.cleanup_tx.send(());
    }

    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }
}

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
            trace!("Incoming connection: {}", id);
            tokio::spawn(
                handle_connection(incoming_session, routes.clone())
                    .instrument(trace_span!("Connection", id)),
            );
        }

        Ok(())
    }
}

async fn handle_connection(incoming_session: IncomingSession, routes: Routes) {
    let (connection_manager, mut cleanup_rx) = ConnectionManager::new();
    let dispatch = Dispatch::new(routes, connection_manager.clone());
    let result = tokio::select! {
        result = dispatch.run(incoming_session) => {
            result
        }
        _ = cleanup_rx.recv() => {
            debug!("Connection cleanup initiated");
            Ok(())
        }
    };

    if let Err(e) = result {
        error!("Connection handler error: {:?}", e);
    }

    connection_manager.mark_disconnected().await;
    debug!("Connection handler completed");
}

use corp_shared::prelude::Colony;
use wtransport::endpoint::ConnectOptions;

struct Dispatch {
    routes: Routes,
    connection_manager: ConnectionManager,
}

impl Dispatch {
    pub fn new(routes: Routes, connection_manager: ConnectionManager) -> Self {
        Self {
            routes,
            connection_manager,
        }
    }

    pub async fn run(&self, session: IncomingSession) -> anyhow::Result<()> {
        let req = session.await?;

        let route = req.headers().get("x-route").cloned().unwrap_or_default();
        let colony = Colony::from_str(route.as_str())?;

        let backend_addr = self
            .get_backend(colony)
            .await
            .ok_or(anyhow::anyhow!("No backend found for route {}", route))?;

        // Connect to backend
        let config = ClientConfig::builder()
            .with_bind_default()
            .with_no_cert_validation()
            .build();

        // Build connect options with all headers from the original request
        let token = req.headers().get("x-token").cloned().unwrap_or_default();
        let connect_options = ConnectOptions::builder(backend_addr)
            .add_header("x-token", token)
            .add_header("x-route", route.clone())
            .build();

        let backend_client = Endpoint::client(config)?.connect(connect_options).await?;
        let frontend_client = req.accept().await?;

        info!("Proxy connection established for route: {}", route);

        // Create cancellation token for all proxy tasks
        let cancellation_token = self.connection_manager.cancellation_token();

        // Spawn proxy tasks with proper cancellation handling
        let bidirectional_task = tokio::spawn(Self::proxy_stream(
            frontend_client.clone(),
            backend_client.clone(),
            cancellation_token.clone(),
        ));

        let client_to_backend_task = tokio::spawn(Self::proxy_client_to_backend(
            frontend_client.clone(),
            backend_client.clone(),
            cancellation_token.clone(),
        ));

        let backend_to_client_task = tokio::spawn(Self::proxy_backend_to_client(
            backend_client.clone(),
            frontend_client.clone(),
            cancellation_token.clone(),
        ));

        let datagram_task = tokio::spawn(Self::proxy_datagrams(
            frontend_client.clone(),
            backend_client.clone(),
            cancellation_token.clone(),
        ));

        // Monitor connection close events and forward them properly
        let close_monitor_task = tokio::spawn(Self::monitor_connection_close(
            frontend_client.clone(),
            backend_client.clone(),
            cancellation_token.clone(),
        ));

        // Wait for either all tasks to complete or cancellation
        tokio::select! {
            result = async {
                tokio::try_join!(
                    bidirectional_task,
                    client_to_backend_task,
                    backend_to_client_task,
                    datagram_task,
                    close_monitor_task
                )
            } => {
                match result {
                    Ok(_) => debug!("All proxy tasks completed normally"),
                    Err(e) => warn!("Proxy task join error: {:?}", e),
                }
            }
            _ = cancellation_token.cancelled() => {
                debug!("Proxy tasks cancelled due to connection termination");
            }
        }

        // Ensure connection is marked as disconnecting
        self.connection_manager.mark_disconnecting().await;

        info!("Proxy connection closed for route: {}", route);
        Ok(())
    }

    async fn proxy_stream(
        frontend: Connection,
        backend: Connection,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        debug!("Starting bidirectional stream proxy");

        loop {
            tokio::select! {
                result = frontend.accept_bi() => {
                    match result {
                        Ok((mut to_client, mut from_client)) => {
                            match backend.open_bi().await?.await {
                                Ok((mut to_backend, mut from_backend)) => {
                                    let token = cancellation_token.clone();
                                    tokio::spawn(async move {
                                        tokio::select! {
                                            _ = async {
                                                // client → backend
                                                let _ = tokio::io::copy(&mut from_client, &mut to_backend).await;
                                                let _ = to_backend.finish().await;

                                                // backend → client
                                                let _ = tokio::io::copy(&mut from_backend, &mut to_client).await;
                                                let _ = to_client.finish().await;
                                            } => {}
                                            _ = token.cancelled() => {
                                                debug!("Stream proxy task cancelled");
                                            }
                                        }
                                    });
                                }
                                Err(e) => {
                                    if Self::is_connection_closed(&e) {
                                        debug!("Backend connection closed while opening bidirectional stream");
                                        break;
                                    }
                                    error!("proxy_stream: failed to open bidi to backend: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            if Self::is_connection_closed(&e) {
                                debug!("Frontend connection closed, ending bidirectional stream proxy");
                                break;
                            }
                            error!("proxy_stream: failed to accept bidi from client: {:?}", e);
                        }
                    }
                }
                _ = cancellation_token.cancelled() => {
                    debug!("Bidirectional stream proxy cancelled");
                    break;
                }
            }
        }

        debug!("Bidirectional stream proxy ended");
        Ok(())
    }

    async fn proxy_client_to_backend(
        frontend: Connection,
        backend: Connection,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        debug!("Starting client-to-backend unidirectional stream proxy");

        loop {
            tokio::select! {
                result = frontend.accept_uni() => {
                    match result {
                        Ok(mut from_client) => {
                            match backend.open_uni().await?.await {
                                Ok(mut to_backend) => {
                                    let token = cancellation_token.clone();
                                    tokio::spawn(async move {
                                        tokio::select! {
                                            _ = async {
                                                let _ = tokio::io::copy(&mut from_client, &mut to_backend).await;
                                                let _ = to_backend.finish().await;
                                            } => {}
                                            _ = token.cancelled() => {
                                                debug!("Client-to-backend stream task cancelled");
                                            }
                                        }
                                    });
                                }
                                Err(e) => {
                                    if Self::is_connection_closed(&e) {
                                        debug!("Backend connection closed while opening unidirectional stream");
                                        break;
                                    }
                                    error!("failed to open uni to backend: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            if Self::is_connection_closed(&e) {
                                debug!("Frontend connection closed, ending client-to-backend stream proxy");
                                break;
                            }
                            error!("failed to accept uni from client: {:?}", e);
                        }
                    }
                }
                _ = cancellation_token.cancelled() => {
                    debug!("Client-to-backend stream proxy cancelled");
                    break;
                }
            }
        }

        debug!("Client-to-backend stream proxy ended");
        Ok(())
    }

    /// Forward every uni‐stream from `backend` → `frontend` in a loop.
    async fn proxy_backend_to_client(
        backend: Connection,
        frontend: Connection,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        debug!("Starting backend-to-client unidirectional stream proxy");

        loop {
            tokio::select! {
                result = backend.accept_uni() => {
                    match result {
                        Ok(mut from_backend) => {
                            match frontend.open_uni().await?.await {
                                Ok(mut to_client) => {
                                    let token = cancellation_token.clone();
                                    tokio::spawn(async move {
                                        tokio::select! {
                                            _ = async {
                                                let _ = tokio::io::copy(&mut from_backend, &mut to_client).await;
                                                let _ = to_client.finish().await;
                                            } => {}
                                            _ = token.cancelled() => {
                                                debug!("Backend-to-client stream task cancelled");
                                            }
                                        }
                                    });
                                }
                                Err(e) => {
                                    if Self::is_connection_closed(&e) {
                                        debug!("Frontend connection closed while opening unidirectional stream");
                                        break;
                                    }
                                    error!("failed to open uni to client: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            if Self::is_connection_closed(&e) {
                                debug!("Backend connection closed, ending backend-to-client stream proxy");
                                break;
                            }
                            error!("failed to accept uni from backend: {:?}", e);
                        }
                    }
                }
                _ = cancellation_token.cancelled() => {
                    debug!("Backend-to-client stream proxy cancelled");
                    break;
                }
            }
        }

        debug!("Backend-to-client stream proxy ended");
        Ok(())
    }

    async fn proxy_datagrams(
        frontend_connection: Connection,
        backend_connection: Connection,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        debug!("Starting datagram proxy");

        // client → backend
        let frontend = frontend_connection.clone();
        let backend = backend_connection.clone();
        let token_c2b = cancellation_token.clone();
        let c2b = tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = frontend.receive_datagram() => {
                        match result {
                            Ok(datagram) => {
                                let _ = backend.send_datagram(datagram.payload());
                            }
                            Err(e) => {
                                if Self::is_connection_closed(&e) {
                                    debug!("Frontend connection closed in datagram proxy (client→backend)");
                                    break;
                                }
                                error!("Error receiving datagram from client: {:?}", e);
                            }
                        }
                    }
                    _ = token_c2b.cancelled() => {
                        debug!("Client-to-backend datagram proxy cancelled");
                        break;
                    }
                }
            }
        });

        // backend → client
        let frontend = frontend_connection.clone();
        let backend = backend_connection.clone();
        let token_b2c = cancellation_token.clone();
        let b2c = tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = backend.receive_datagram() => {
                        match result {
                            Ok(datagram) => {
                                let _ = frontend.send_datagram(datagram.payload());
                            }
                            Err(e) => {
                                if Self::is_connection_closed(&e) {
                                    debug!("Backend connection closed in datagram proxy (backend→client)");
                                    break;
                                }
                                error!("Error receiving datagram from backend: {:?}", e);
                            }
                        }
                    }
                    _ = token_b2c.cancelled() => {
                        debug!("Backend-to-client datagram proxy cancelled");
                        break;
                    }
                }
            }
        });

        // Wait for both datagram proxy tasks to complete or be cancelled
        tokio::select! {
            result = async {
                tokio::try_join!(c2b, b2c)
            } => {
                match result {
                    Ok(_) => debug!("Datagram proxy tasks completed normally"),
                    Err(e) => warn!("Datagram proxy task join error: {:?}", e),
                }
            }
            _ = cancellation_token.cancelled() => {
                debug!("Datagram proxy cancelled");
            }
        }

        debug!("Datagram proxy ended");
        Ok(())
    }

    /// Monitor connection close events and forward them properly
    /// This is critical for handling REQUEST_DISCONNECT and other graceful closes
    async fn monitor_connection_close(
        frontend: Connection,
        backend: Connection,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        debug!("Starting connection close monitoring");

        // Create tasks to monitor each connection's close event
        let frontend_clone = frontend.clone();
        let backend_clone = backend.clone();
        let token_clone = cancellation_token.clone();

        let frontend_monitor = tokio::spawn(async move {
            frontend_clone.closed().await;
            debug!("Frontend connection closed detected");
        });

        let backend_monitor = tokio::spawn(async move {
            backend_clone.closed().await;
            debug!("Backend connection closed detected");
        });

        tokio::select! {
            // Frontend connection closed
            _ = frontend_monitor => {
                debug!("Frontend connection closed, forwarding close to backend");
                // Forward the close to the backend with graceful close code
                backend.close(VarInt::from_u32(0), b"client disconnected");
                backend.closed().await;
                debug!("Backend connection closed in response to frontend close");
            }
            // Backend connection closed
            _ = backend_monitor => {
                debug!("Backend connection closed, forwarding close to frontend");
                // Forward the close to the frontend
                frontend.close(VarInt::from_u32(0), b"server disconnected");
                frontend.closed().await;
                debug!("Frontend connection closed in response to backend close");
            }
            // Handle cancellation
            _ = token_clone.cancelled() => {
                debug!("Connection close monitoring cancelled");
                // Close both connections gracefully
                frontend.close(VarInt::from_u32(0), b"proxy shutdown");
                backend.close(VarInt::from_u32(0), b"proxy shutdown");

                // Wait for both to close
                tokio::join!(frontend.closed(), backend.closed());
                debug!("Both connections closed due to cancellation");
            }
        }

        debug!("Connection close monitoring ended");
        Ok(())
    }

    /// Helper method to detect if an error indicates a closed connection
    /// This method properly distinguishes between graceful closes and connection errors
    fn is_connection_closed(error: &dyn std::error::Error) -> bool {
        let error_str = error.to_string().to_lowercase();

        // Check for graceful application close first
        if error_str.contains("application closed") || error_str.contains("applicationclosed") {
            debug!("Connection closed gracefully by application");
            return true;
        }

        // Check for various connection closure indicators
        error_str.contains("closed")
            || error_str.contains("connection reset")
            || error_str.contains("connection aborted")
            || error_str.contains("broken pipe")
            || error_str.contains("not connected")
            || error_str.contains("connection lost")
            || error_str.contains("session closed")
            || error_str.contains("stream closed")
            || error_str.contains("no route to host")
            || error_str.contains("connection refused")
            || error_str.contains("timed out")
            || error_str.contains("connection terminated")
    }

    async fn get_backend(&self, world_id: Colony) -> Option<String> {
        let routes = self.routes.read().await;
        routes.get(&world_id).cloned()
    }
}
