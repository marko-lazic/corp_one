use crate::{table, ServerState};
use bevy::{prelude::*, tasks::IoTaskPool};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;

pub struct DbPlugin;

impl Plugin for DbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ServerState::Load), database_setup)
            .add_systems(OnEnter(ServerState::Serve), light_year_and_session);
    }
}

fn database_setup(/*mut commands: Commands*/) {
    let pool = Arc::new(
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_lazy("sqlite:corp_server/db/database.sqlite")
            .expect("Failed to create database pool"),
    );

    // commands.insert_resource(pool.clone());

    IoTaskPool::get()
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

fn light_year_and_session() {
    // Testing with lightyear crate.
}
