use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginConfig {
    #[serde(default)]
    pub host: String,

    #[serde(default)]
    pub port: u16,

    #[serde(default)]
    pub database_url: String,

    #[serde(default)]
    pub migrations_dir: PathBuf,

    #[serde(default)]
    pub nats_url: String,
}

impl LoginConfig {
    pub fn for_testing() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 15550,
            database_url: "sqlite::memory:".to_string(),
            migrations_dir: PathBuf::from("../corp_login/migrations"),
            nats_url: "nats://localhost:50961".to_string(),
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_database_url(&self) -> String {
        self.database_url.clone()
    }

    pub fn get_migrations_dir(&self) -> PathBuf {
        self.migrations_dir.clone()
    }
}

impl Default for LoginConfig {
    fn default() -> Self {
        let dirs = corp_types::dirs::Dirs::new("corp_login");
        let database_path = dirs.data_dir.join("corp_login.sqlite");
        let database_url = format!("sqlite:{}", database_path.to_string_lossy());

        Self {
            host: "127.0.0.1".to_string(),
            port: 25550,
            database_url,
            migrations_dir: PathBuf::from("corp_login/migrations"),
            nats_url: "nats://localhost:4222".to_string(),
        }
    }
}
