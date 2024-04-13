use diesel::prelude::*;

use super::establish_connection;
use super::models::TwitchBot;
use super::schema::twitch_bot;

pub async fn bot_owner(bot_id: &i32) -> i32 {
    let connection = &mut establish_connection();
    let bot = twitch_bot::table
        .filter(twitch_bot::id.eq(bot_id))
        .select(TwitchBot::as_select())
        .get_result(connection)
        .unwrap();
    bot.channel_id
}

pub async fn bot_owner_from_state(state: &str) -> i32 {
    let connection = &mut establish_connection();
    let bot = twitch_bot::table
        .filter(twitch_bot::state.eq(state))
        .select(TwitchBot::as_select())
        .get_result(connection)
        .unwrap();
    bot.channel_id
}
