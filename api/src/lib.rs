use std::env;
use axum::{routing::get, Router};
use migration::MigratorTrait;
use sea_orm::Database;

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

    let app = Router::new().route("/", get(|| async { "Hellow word" }));

    let listener = tokio::net::TcpListener::bind(app_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}