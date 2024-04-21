//! Get information about a channel

use diesel::{prelude::*, result};

use crate::establish_connection;
use crate::models::TwitchBot;
use crate::schema::twitch_bot;

pub async fn bot_owner(bot_id: &i32) -> Result<TwitchBot, result::Error> {
    twitch_bot::table
        .filter(twitch_bot::id.eq(bot_id))
        .select(TwitchBot::as_select())
        .get_result(&mut establish_connection())
}

pub async fn bot_owner_from_state(state: &str) -> Result<TwitchBot, result::Error> {
    println!("state: {}", state);
    twitch_bot::table
        .filter(twitch_bot::state.eq(state))
        .select(TwitchBot::as_select())
        .get_result(&mut establish_connection())
}
