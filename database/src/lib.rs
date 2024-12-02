use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

use types::get::database;

pub mod bot;
pub mod channel;
pub mod login;
pub mod models;
pub mod responder;

mod schema;

/// Creates a new connection to the database
// TODO: I *think* these connections will drop when the function using them is over, but I should test that
fn establish_connection() -> SqliteConnection {
    let db = database(None);
    SqliteConnection::establish(&db.url).unwrap_or_else(|_| panic!("Error connecting to {}", db.url))
}
