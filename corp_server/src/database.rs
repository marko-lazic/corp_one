use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::{Datetime, Thing},
    Surreal,
};
use surrealdb_migrations::MigrationRunner;

use crate::ServerState;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    registered_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
struct Faction {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Colony {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct DbPlugin;

impl Plugin for DbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ServerState::Load), database_setup)
            .add_systems(OnEnter(ServerState::Serve), print_data);
    }
}

fn database_setup(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        dotenv().ok();
        let db_user = std::env::var("DB_USER").expect("DB_USER must be set.");
        let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
        DB.connect::<Ws>("localhost:8000")
            .await
            .expect("Failed to connect");

        DB.signin(Root {
            username: &db_user,
            password: &db_password,
        })
        .await
        .expect("Could not sign in");

        DB.use_ns("test")
            .use_db("test")
            .await
            .expect("failed to set ns or db");

        // Apply all migrations
        MigrationRunner::new(&DB)
            .up()
            .await
            .expect("Failed to apply migrations");

        // Change state if database initialization completes successfully
        ctx.run_on_main_thread(move |ctx| {
            let world: &mut World = ctx.world;
            world
                .get_resource_mut::<NextState<ServerState>>()
                .unwrap()
                .set(ServerState::Serve);
            info!("Entering Serve state");
        })
        .await;
    });
}

fn print_data(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|_ctx| async move {
        print_colonies().await;
        print_users().await;
    });
}

async fn print_colonies() {
    let colonies: Vec<Colony> = match DB.select("colony").await {
        Ok(data) => data,
        Err(error) => {
            error!("Error fetching colonies: {:?}", error);
            return;
        }
    };

    let colonies: String = colonies
        .iter()
        .map(|colony| colony.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    info!("Colonies: {}", colonies);
}

async fn print_users() {
    let users: Vec<User> = match DB.select("user").await {
        Ok(data) => data,
        Err(error) => {
            error!("Error fetching users: {:?}", error);
            return;
        }
    };

    let users: String = users
        .iter()
        .map(|user| format!("{} {}", user.username.clone(), user.email.clone()))
        .collect::<Vec<String>>()
        .join(", ");
    info!("Users: {}", users);
}
