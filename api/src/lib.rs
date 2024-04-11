use std::env;
use axum::{routing::get, Router};
use sea_orm::Database;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let database_env_connection = env::var("DATABASE_CONNECTION")
        .expect("DATABASE_CONNECTION is not defined on .env file");
    let app_port = env::var("APP_PORT")
        .expect("APP_PORT is not defined on .env file");

    let database_connection = Database::connect(database_env_connection)
        .await
        .expect("Database connection failed");

    let app = Router::new().route("/", get(|| async { "Hellow word" }));

    let listener = tokio::net::TcpListener::bind(app_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}