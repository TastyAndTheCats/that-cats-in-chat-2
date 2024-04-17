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
use dotenvy::dotenv;
use std::env;

pub mod bot;
pub mod channel;
pub mod login;
pub mod models;

mod schema;

use models::TwitchBot;
use schema::{twitch_login_process, twitch_user};

use crate::models::TwitchLoginAccessToken;

/// Creates a new connection to the database
// TODO: I *think* these connections will drop when the function using them is over, but I should test that
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn twitch_access_token() -> Option<String> {
    let connection = &mut establish_connection();
    let access_token_wrapper = twitch_user::table
        .inner_join(twitch_login_process::table)
        .filter(twitch_user::id.eq(167591621))
        .select(TwitchLoginAccessToken::as_select())
        .get_result(connection)
        .unwrap();
    access_token_wrapper.access_token
}
