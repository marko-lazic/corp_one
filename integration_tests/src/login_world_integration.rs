#[cfg(test)]
mod tests {
    use crate::nats;
    use async_nats::jetstream;
    use corp_login::LoginConfig;
    use corp_types::prelude::{CreateUserRequest, User};
    use corp_world::WorldConfig;
    use std::time::Duration;
    use tracing::info;

    #[tokio::test]
    async fn test_register_user_creates_player() -> anyhow::Result<()> {
        init_tracing();
        let p = nats::run_basic_server();
        setup_nats_streams(p.client_url()).await?;

        info!("Start NATS Server {:?}", p.client_url());

        let login_handle = tokio::spawn(async move {
            info!("Hello world! Starting login service");
            match corp_login::run_login_with_config(LoginConfig::for_testing()).await {
                Ok(_) => {}
                Err(e) => tracing::error!("Login service error: {}", e),
            }
        });

        let world_handle = tokio::spawn(async move {
            match corp_world::run_world_with_config(WorldConfig::for_testing()).await {
                Ok(_) => {}
                Err(e) => tracing::error!("World service error: {}", e),
            }
        });

        wait_for_service_ready(&LoginConfig::for_testing().bind_address()).await?;

        let result = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?
            .post(&format!(
                "http://{}/register",
                LoginConfig::for_testing().bind_address()
            ))
            .json(&CreateUserRequest {
                username: "testuser".to_string(),
                email: "user@test.com".to_string(),
                password: "testpass".to_string(),
            })
            .send()
            .await?
            .json::<User>()
            .await?;

        info!("Response: {:?}", result);
        assert_eq!(result.username, "testuser");
        assert_eq!(result.email, "user@test.com");

        login_handle.abort();
        world_handle.abort();

        Ok(())
    }

    async fn wait_for_service_ready(bind_address: &str) -> anyhow::Result<()> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()?;

        let mut interval = tokio::time::interval(Duration::from_millis(100));
        let max_attempts = 100; // 10 seconds total

        for _ in 0..max_attempts {
            interval.tick().await;

            if let Ok(response) = client
                .get(&format!("http://{}/health", bind_address))
                .send()
                .await
            {
                if response.status().is_success() {
                    return Ok(());
                }
            }
        }

        Err(anyhow::anyhow!("Service failed to start within timeout"))
    }

    fn init_tracing() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .init();
    }

    async fn setup_nats_streams(nats_url: impl Into<String>) -> anyhow::Result<()> {
        let client = async_nats::connect(nats_url.into()).await?;
        let jetstream = jetstream::new(client);

        // Just create the USER_EVENTS stream - this is the minimum needed
        let stream_config = jetstream::stream::Config {
            name: "USER_EVENTS".to_string(),
            subjects: vec!["user.>".to_string()],
            ..Default::default()
        };

        jetstream.create_stream(stream_config).await?;

        Ok(())
    }
}
