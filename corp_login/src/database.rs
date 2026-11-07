use crate::LoginConfig;
use anyhow::{Result, anyhow};
use sqlx::{AssertSqlSafe, SqlitePool, sqlite::SqlitePoolOptions};
use std::{env, ffi::OsStr, path::Path};
use tracing::info;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    info!("Connected to SQLite database");
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool, config: &LoginConfig) -> Result<()> {
    info!("Running database migrations...");

    let migrations_dir = env::current_dir()?.join(config.get_migrations_dir());
    info!("Login migrations dir: {:?}", migrations_dir);
    if !migrations_dir.exists() {
        return Err(anyhow!(
            "Login migrations dir not found: {:?}",
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
        info!("Running migration: {:?}", migration_file);
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
                let statement = current_statement.trim_end_matches(';').trim().to_owned();
                if !statement.is_empty() {
                    sqlx::query(AssertSqlSafe(statement)).execute(pool).await?;
                }
                current_statement.clear();
            }
        }

        // Handle any remaining statement without semicolon
        if !current_statement.trim().is_empty() {
            let statement = current_statement.trim().to_owned();
            sqlx::query(AssertSqlSafe(statement)).execute(pool).await?;
        }

        info!("Migration completed: {:?}", migration_file);
    }

    info!("All migrations completed successfully");
    Ok(())
}

pub async fn setup_database(config: LoginConfig) -> Result<SqlitePool> {
    let database_url = config.get_database_url();

    info!("Creating database: {}", database_url);

    // Skip file operations for in-memory databases
    if !database_url.contains("::memory:") && !database_url.contains(":memory:") {
        let file_path = if database_url.starts_with("sqlite:") {
            database_url
                .strip_prefix("sqlite:")
                .unwrap_or(&database_url)
                .trim_start_matches("//") // Remove potential // after sqlite:
        } else {
            &database_url
        };

        info!("Database file path: {}", file_path);

        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(file_path).parent()
            && !parent.exists()
        {
            info!("Creating directory structure: {:?}", parent);
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                anyhow::anyhow!("Failed to create directory structure {:?}: {}", parent, e)
            })?;
        }

        // Create the database file if it doesn't exist
        if !Path::new(file_path).exists() {
            info!("Creating database file: {}", file_path);
            tokio::fs::File::create(file_path).await.map_err(|e| {
                anyhow::anyhow!("Failed to create database file {}: {}", file_path, e)
            })?;
        }
    }

    let pool = create_pool(&database_url).await?;
    run_migrations(&pool, &config).await?;
    Ok(pool)
}
