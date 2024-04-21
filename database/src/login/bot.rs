//! Routes we use when logging in a bot

use diesel::{prelude::*, result};

use crate::establish_connection;
use crate::models::{LoginProcess, TwitchBot};
use crate::schema::{twitch_bot, twitch_login_process};

/// Takes the channel_id and returns the TwitchBot object associated with it
pub async fn bot_from_owner_id(owner_id: &i32) -> Result<TwitchBot, result::Error> {
    let connection = &mut establish_connection();
    twitch_bot::table
        .filter(twitch_bot::channel_id.eq(owner_id))
        .select(TwitchBot::as_select())
        .get_result(connection)
}

/// Takes the login state and returns the associated LoginProcess
pub async fn bot_access_token(state: &str) -> Result<LoginProcess, result::Error> {
    let connection = &mut establish_connection();
    twitch_login_process::table
        .filter(twitch_login_process::state.eq(state))
        .select(LoginProcess::as_select())
        .get_result(connection)
}

/// Updates the LoginProcess for a given Login state
pub async fn update_bot_access_token(
    state: &str,
    new_login_process: LoginProcess,
) -> Result<LoginProcess, result::Error> {
    let connection = &mut establish_connection();
    diesel::update(twitch_login_process::table.find(state))
        .set((
            twitch_login_process::access_token.eq(new_login_process.access_token),
            twitch_login_process::refresh_token.eq(new_login_process.refresh_token),
            twitch_login_process::initiated_at.eq(new_login_process.initiated_at),
            twitch_login_process::token_expiry.eq(new_login_process.token_expiry),
        ))
        .returning(LoginProcess::as_returning())
        .get_result(connection)
}

/// After validation, twitch hands back the user id and login, thesse are useful, so we save them
pub fn save_initial_bot_details(
    state: &str,
    id: &i32,
    login: &str,
    bot_owner: &i32,
) -> Result<usize, result::Error> {
    let connection = &mut establish_connection();

    // Delete any old logins/attempts for this channel (one bot per channel)
    diesel::delete(twitch_bot::table.filter(twitch_bot::channel_id.eq(bot_owner)))
        .execute(connection)
        .expect("Unable to delete other associated bots");

    // Then write a new record since we are given all the info we need
    diesel::insert_into(twitch_bot::table)
        .values((
            twitch_bot::id.eq(id),
            twitch_bot::login.eq(login),
            twitch_bot::state.eq(state),
            twitch_bot::channel_id.eq(bot_owner),
        ))
        // There shouldn't be any collisions given the above deletion
        .execute(connection)
}

// Set the channel_id for the owner's channel to associate the two during the bot login process
pub fn add_bot_owner(state: &str, owner_id: &i32) -> Result<usize, result::Error> {
    let connection = &mut establish_connection();
    diesel::insert_into(twitch_bot::table)
        .values((
            twitch_bot::state.eq(state),
            twitch_bot::channel_id.eq(owner_id),
        ))
        // .on_conflict(twitch_bot::channel_id)
        // .do_update()
        // .set(twitch_bot::state.eq(state))
        .execute(connection)
}
