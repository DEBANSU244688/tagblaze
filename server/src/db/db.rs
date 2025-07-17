use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    /// Retrieves the database connection URL from the environment variable `DATABASE_URL`.
    ///
    /// # Panics
    ///
    /// Panics with the message "Missing DATABASE_URL" if the environment variable is not set.
    ///
    /// # Example
    ///
    /// ```rust
    /// let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    /// ```
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    Database::connect(&db_url)
        .await
        .expect("❌ Failed to connect to database")
}
