use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct WorldConfig {
    #[serde(default)]
    pub testing_mode: bool,

    #[serde(default)]
    pub migrations_dir: Option<PathBuf>,

    #[serde(default)]
    pub nats_url: String,
}

impl WorldConfig {
    /// Load configuration from environment variables with CORP_WORLD_ prefix
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::prefixed("CORP_WORLD_").from_env::<Self>()
    }

    /// Create configuration for testing (in-memory database)
    pub fn for_testing() -> Self {
        Self {
            testing_mode: true,
            migrations_dir: Some(PathBuf::from("../corp_world/migrations")),
            nats_url: "nats://localhost:50961".to_string(),
        }
    }

    /// Get database URL - in-memory if testing_mode, otherwise file-based
    pub fn get_database_url(&self) -> String {
        if self.testing_mode {
            "sqlite::memory:".to_string()
        } else {
            // Use default file-based database
            let dirs = corp_types::dirs::Dirs::new("corp_world");
            let database_path = dirs.data_dir.join("corp_world.sqlite");
            format!("sqlite:{}", database_path.to_string_lossy())
        }
    }

    /// Get migrations directory path
    pub fn get_migrations_dir(&self) -> PathBuf {
        if let Some(dir) = &self.migrations_dir {
            dir.clone()
        } else {
            // Use existing logic for migration directory detection
            if std::path::Path::new("migrations").exists() {
                PathBuf::from("migrations")
            } else {
                PathBuf::from("corp_world/migrations")
            }
        }
    }
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            testing_mode: false,
            migrations_dir: None,
            nats_url: "nats://localhost:4222".to_string(),
        }
    }
}
