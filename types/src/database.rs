use std::env;

/// From Environment Variables
#[derive(Debug)]
pub struct PostgresDatabase {
    // pub username: String,
    // pub password: String,
    // pub database: String,
    pub url: String,
}

impl Default for PostgresDatabase {
    fn default() -> Self {
        PostgresDatabase {
            url: env::var("DATABASE_URL").expect("DATABASE_URL is missing"),
        }
    }
}
