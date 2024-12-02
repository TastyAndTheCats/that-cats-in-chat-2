use std::env;

/// From Environment Variables
#[derive(Debug)]
pub struct SqliteDatabase {
    pub url: String,
}

impl Default for SqliteDatabase {
    fn default() -> Self {
        SqliteDatabase {
            url: env::var("DATABASE_URL").expect("DATABASE_URL is missing"),
        }
    }
}
