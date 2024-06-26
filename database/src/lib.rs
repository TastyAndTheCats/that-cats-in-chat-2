//! Database functions for TCIC2
//!
//! Everything that uses the db needs to use this establish_connection fn and it's not public,
//! so every database request necessarily must be inside this library
//!
//! I expect there to be many modules containing many more functions that enable the functionality of the chatbot
//! and I didn't want to have to dig through them all when things needed to be changed,
//! and I wanted to at least have a chance at reusing some of these database functions

// TODO: these db functions don't return anything, but they should probably all return Result<(), Error>
// so there aren't any panics buuut we're just going with the happy path right now

use diesel::pg::PgConnection;
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
fn establish_connection() -> PgConnection {
    let db = database(None);
    PgConnection::establish(&db.url).unwrap_or_else(|_| panic!("Error connecting to {}", db.url))
}
