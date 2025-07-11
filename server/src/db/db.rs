use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    Database::connect(&db_url)
        .await
        .expect("❌ Failed to connect to database")
}