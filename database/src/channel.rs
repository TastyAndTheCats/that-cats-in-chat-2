//! Get information about a channel

use diesel::{prelude::*, result};

use crate::{
    establish_connection,
    models::TwitchUser,
    schema::twitch_user,
};
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
