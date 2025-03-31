use crate::{dirs::Dirs, game::*, table};
use bevy::{prelude::*, tasks::IoTaskPool};
use corp_shared::prelude::Colony;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;

pub struct DbPlugin;

impl Plugin for DbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, database_setup.run_if(is_colony_iris));
    }
}

fn is_colony_iris(config: Res<GameInstanceConfig>) -> bool {
    config.colony == Colony::Iris
}

fn database_setup() {
    let dirs = Dirs::new("corp_server");
    let database_path_buf = dirs.data_dir.join("database.sqlite");
    let _ignored = dirs.config_dir;
    let _ignored = dirs.cache_dir;
    let database_path_str = database_path_buf
        .to_str()
        .expect("Path is not valid UTF-8, cannot form database URL");
    let database_url = format!("sqlite:{}", database_path_str);
    let pool = Arc::new(
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_lazy(&database_url)
            .expect("Failed to create database pool"),
    );

    // commands.insert_resource(pool.clone());
    let io_pool = IoTaskPool::get();
    io_pool
        .spawn(async move {
            match fetch_players(pool).await {
                Ok(players) => {
                    for player in players {
                        println!("{:?}", player);
                    }
                }
                Err(e) => eprintln!("Failed to fetch players: {:?}", e),
            }
        })
        .detach();
}

async fn fetch_players(pool: Arc<SqlitePool>) -> Result<Vec<table::Player>, sqlx::Error> {
    let players = sqlx::query_as::<_, table::Player>("SELECT * FROM players")
        .fetch_all(&*pool)
        .await?;
    Ok(players)
}
