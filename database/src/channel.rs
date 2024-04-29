//! Get information about a channel

use diesel::{prelude::*, result};

use crate::{
    establish_connection,
    models::{TwitchBot, TwitchUser},
    schema::{twitch_bot, twitch_user},
};

pub fn bot_owner(bot_id: &i32) -> Result<TwitchBot, result::Error> {
    twitch_bot::table
        .filter(twitch_bot::id.eq(bot_id))
        .select(TwitchBot::as_select())
        .get_result(&mut establish_connection())
}

pub fn bot_owner_from_state(state: &str) -> Result<TwitchBot, result::Error> {
    tracing::debug!("state: {}", state);
    twitch_bot::table
        .filter(twitch_bot::state.eq(state))
        .select(TwitchBot::as_select())
        .get_result(&mut establish_connection())
}

pub fn increment_messages_counted(user_id: i32) -> Result<TwitchUser, result::Error> {
    use crate::schema::twitch_user::dsl::*;
    diesel::update(twitch_user.filter(id.eq(user_id)))
        .set(messages_processed.eq(messages_processed + 1))
        .returning(TwitchUser::as_returning())
        .get_result(&mut establish_connection())
}

pub fn get_messages_counted(user_id: i32) -> Result<i32, result::Error> {
    twitch_user::table
        .filter(twitch_user::id.eq(user_id))
        .select(twitch_user::messages_processed)
        .get_result(&mut establish_connection())
}
