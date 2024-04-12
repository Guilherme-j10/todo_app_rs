mod entity;

use std::env;
use anyhow::Ok;
use axum::{
    extract::{Json, State}, response::IntoResponse, routing::{get, post}, Router
};
use migration::MigratorTrait;
use sea_orm::{ ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, EntityTrait, QueryOrder };
use entity::todo_table::{self, Entity as TodoTable};
use serde::Deserialize;
use ulid::Ulid;
use chrono::Local;
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
struct CreateTask {
    description: String,
}

#[derive(Clone)]
struct AppState {
    database_connection: DatabaseConnection
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let database_env_connection = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not defined on .env file");
    let app_port = env::var("APP_PORT")
        .expect("APP_PORT is not defined on .env file");

    let database_connection = Database::connect(database_env_connection)
        .await
        .expect("Database connection failed");

    //up all migrations
    migration::Migrator::up(&database_connection, None).await?;
    
    if database_connection.ping().await.is_ok() {
        println!("Database connection is ok");
    }

    let state = AppState {
        database_connection
    };

    let app = Router::new()
        .route("/", get(|| async { "Hellow word" }))
        .route("/create_task", post(create_task))
        .route("/list_tasks", get(list_tasks))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(app_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn list_tasks(State(state): State<AppState>) -> impl IntoResponse {
    let tasks = TodoTable::find()
        .order_by_desc(todo_table::Column::Id)
        .all(&state.database_connection)
        .await
        .expect("Erro in list task");

    Json::from(tasks)
}

async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>
) -> Json<Value> {
    let ulid = Ulid::new();
    let now = Local::now().naive_local();

    let task = entity::todo_table::ActiveModel {
        description: Set(payload.description.to_owned()),
        id: Set(ulid.to_string().to_owned()),
        status: Set("undone".to_owned()),
        create_at: Set(now.to_owned()),
        update_at: Set(now.to_owned())
    };

    task.insert(&state.database_connection).await.unwrap();
    Json(json!(true))
}