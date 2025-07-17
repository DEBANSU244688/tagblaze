// use std::env;

// pub struct Config {
//     pub database_url: String,
// }

// impl Config {
//     pub fn from_env() -> Self {
//         dotenvy::dotenv().ok(); // You’ll add `.env` later
//         let database_url =
//             env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
//         Self { database_url }
//     }
// }