use crate::WorldConfig;
use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::{ffi::OsStr, path::Path};
use tracing::{error, info};

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    info!("Connected to corp_world SQLite database: {}", database_url);
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool, config: &WorldConfig) -> Result<()> {
    info!("Running corp_world database migrations...");

    let migrations_dir = config.get_migrations_dir();

    if !migrations_dir.exists() {
        error!(
            "Corp_world migrations directory not found: {:?}",
            migrations_dir
        );
        return Err(anyhow::anyhow!(
            "Corp_world migrations directory not found: {:?}",
            migrations_dir
        ));
    }

    let mut entries = tokio::fs::read_dir(&migrations_dir).await?;
    let mut migration_files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension() == Some(OsStr::new("sql")) {
            migration_files.push(path);
        }
    }

    migration_files.sort();

    for migration_file in migration_files {
        info!("Running corp_world migration: {:?}", migration_file);
        let content = tokio::fs::read_to_string(&migration_file).await?;

        // Process each line or statement block
        let lines: Vec<&str> = content
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && !s.starts_with("--"))
            .collect();

        let mut current_statement = String::new();

        for line in lines {
            current_statement.push_str(line);
            current_statement.push(' ');

            if line.ends_with(';') {
                let statement = current_statement.trim_end_matches(';').trim();
                if !statement.is_empty() {
                    sqlx::query(statement).execute(pool).await?;
                }
                current_statement.clear();
            }
        }

        // Handle any remaining statement without semicolon
        if !current_statement.trim().is_empty() {
            let statement = current_statement.trim();
            sqlx::query(statement).execute(pool).await?;
        }

        info!("Corp_world migration completed: {:?}", migration_file);
    }

    info!("All corp_world migrations completed successfully");
    Ok(())
}

pub async fn setup_database(config: WorldConfig) -> Result<SqlitePool> {
    let database_url = config.get_database_url();

    // Create database file if it's a file-based database and doesn't exist
    if !config.testing_mode && !database_url.contains(":memory:") {
        let db_path = database_url
            .strip_prefix("sqlite:")
            .unwrap_or(&database_url);
        if !Path::new(db_path).exists() {
            info!("Creating corp_world database file: {}", db_path);
            if let Some(parent) = Path::new(db_path).parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            tokio::fs::File::create(db_path).await?;
        }
    }

    let pool = create_pool(&database_url).await?;
    run_migrations(&pool, &config).await?;
    Ok(pool)
}
