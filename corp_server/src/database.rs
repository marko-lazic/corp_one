use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, Mem},
    sql::Thing,
    Surreal,
};

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[derive(Debug, Serialize, Deserialize)]
struct Faction {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Colony {
    title: String,
    owner: Faction,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct DbPlugin;

impl Plugin for DbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, database_setup)
            .add_systems(Update, list_colony_data);
    }
}

fn database_setup(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|_ctx| async move {
        DB.connect::<Mem>(()).await.unwrap();
        DB.use_ns("test").use_db("test").await.unwrap();

        let _: Option<Record> = DB
            .create(("colony", "liberte"))
            .content(Colony {
                title: "Liberte".to_string(),
                owner: Faction {
                    title: "EC".to_string(),
                },
            })
            .await
            .unwrap();
    });
}

fn list_colony_data(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|_ctx| async move {
        let colonies: surrealdb::Result<Vec<Colony>> = DB.select("colony").await;
        if let Ok(colonies) = colonies {
            for colony in colonies {
                dbg!(colony);
            }
        }
    });
}
