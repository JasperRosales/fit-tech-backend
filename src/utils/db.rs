use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn init_db() -> DatabaseConnection {
    dotenvy::from_filename("local.env").ok();

    let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_pass = env::var("DB_PASS").expect("DB_PASS must be set");
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name
    );

    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
