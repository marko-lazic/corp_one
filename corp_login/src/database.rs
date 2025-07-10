use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use anyhow::Result;
use std::path::Path;
use tracing::{info, error};

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    
    info!("Connected to SQLite database");
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    info!("Running database migrations...");
    
    let migrations_dir = if Path::new("migrations").exists() {
        Path::new("migrations")
    } else if Path::new("corp_login/migrations").exists() {
        Path::new("corp_login/migrations")
    } else {
        error!("Migrations directory not found");
        return Err(anyhow::anyhow!("Migrations directory not found"));
    };
    
    let mut entries = tokio::fs::read_dir(migrations_dir).await?;
    let mut migration_files = Vec::new();
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "sql") {
            migration_files.push(path);
        }
    }
    
    migration_files.sort();
    
    for migration_file in migration_files {
        info!("Running migration: {:?}", migration_file);
        let content = tokio::fs::read_to_string(&migration_file).await?;
        
        // Process each line or statement block
        let lines: Vec<&str> = content.lines()
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
        
        info!("Migration completed: {:?}", migration_file);
    }
    
    info!("All migrations completed successfully");
    Ok(())
}

pub async fn setup_database() -> Result<SqlitePool> {
    let database_url = "sqlite:corp_login.db";
    
    // Create database if it doesn't exist
    if !std::path::Path::new("corp_login.db").exists() {
        info!("Creating database file: corp_login.db");
        tokio::fs::File::create("corp_login.db").await?;
    }
    
    let pool = create_pool(database_url).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}