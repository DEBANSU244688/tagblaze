/// Establishes a connection to the database using the URL specified in the `DATABASE_URL` environment variable.
/// 
/// This function loads environment variables from a `.env` file (if present) and attempts to connect to the database.
/// 
/// # Panics
/// 
/// Panics if the `DATABASE_URL` environment variable is not set or if the connection to the database fails.
/// 
/// # Returns
/// 
/// Returns an active [`DatabaseConnection`] on success.
/// 
/// # Example
/// 
/// ```rust
/// let connection = connect().await;
/// ```

use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    Database::connect(&db_url)
        .await
        .expect("❌ Failed to connect to database")
}
