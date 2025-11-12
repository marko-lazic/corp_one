use crate::{
    channel::ChannelTriggerApp,
    integration::{TokenService, RETRY_DELAY},
    server::*,
};
use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::*,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use corp_shared::network::TICK_RATE;
use corp_types::{messenger::Messenger, prelude::Colony};
use std::time::Duration;
use tokio::runtime::Runtime;

pub struct GameServer {
    game_server_config: GameServerConfig,
    runtime: Runtime,
}

impl GameServer {
    pub fn new(game_server_config: GameServerConfig) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime");
        Self {
            game_server_config,
            runtime,
        }
    }
    pub fn create_app(&self) -> App {
        let wait_duration = Duration::from_secs_f64(1.0 / f64::from(TICK_RATE));
        let mut app = App::new();
        app.insert_resource(self.game_server_config.clone())
            .add_plugins((
                MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(wait_duration)),
                TokensPlugin,
                StatesPlugin,
                ServerNetPlugin,
                EntropyPlugin::<WyRand>::default(),
            ));
        if self.is_colony_server() {
            app.add_plugins((
                LootPlugin,
                HealthRemotePlugin,
                DeathPlugin,
                CloningRemotePlugin,
                PlayersPlugin,
            ));
        }
        app
    }

    pub fn run(&self) {
        let mut app = self.create_app();
        self.run_async_server(&mut app);
        app.run();
    }

    fn run_async_server(&self, app: &mut App) {
        let bevy_sender = app.add_channel_trigger::<BevyUserEvent>();
        let handle = self.runtime.handle().clone();
        let game_server_config = self.game_server_config.clone();

        handle.spawn(async move {
            let nats_url = "nats://localhost:4222";
            info!("Starting NATS connection: {}", nats_url);
            let nc = GameServer::create_nats_client(nats_url).await;
            let messenger = Messenger::new(nc);

            let channel_name = format!("game_server_{}", game_server_config.colony.to_snake_case());

            let mut token_service = TokenService::new(bevy_sender, messenger, channel_name);
            token_service.run().await;
        });
    }

    async fn create_nats_client(nats_url: &str) -> async_nats::Client {
        loop {
            match async_nats::connect(nats_url).await {
                Ok(connection) => {
                    info!("NATS connection established");
                    return connection;
                }
                Err(e) => {
                    error!("Failed to connect to NATS: {}", e);
                    info!("Attempting to reconnect to NATS...");
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    }

    fn is_colony_server(&self) -> bool {
        self.game_server_config.colony != Colony::StarMap
    }
}
