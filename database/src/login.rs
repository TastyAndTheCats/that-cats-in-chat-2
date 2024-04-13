use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use super::establish_connection;
use super::models::LoginProcess;
use super::schema::{twitch_bot, twitch_login_process, twitch_user};

/// The admin site uses this method to login as a broadcaster, bot, or both
pub fn initiate_login(state: &str, scope: &str, is_broadcaster: bool, is_bot: bool) {
    let login = LoginProcess {
        state: state.to_string(),
        scope: scope.to_string(),
        code: None,
        is_broadcaster: is_broadcaster,
        is_bot: is_bot,
        initiated_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0), // TODO: make better
        refresh_token: None,
        token_expiry: None,
        access_token: None,
        token_type: None,
    };

    let connection = &mut establish_connection();
    diesel::insert_into(twitch_login_process::table)
        .values(&login)
        .returning(LoginProcess::as_returning())
        .get_result(connection)
        .expect("Error saving new post");
}

/// The User has authorized the app
pub fn twitch_login_successful(state: &str, scope: &str, code: &str) {
    let connection = &mut establish_connection();
    diesel::update(twitch_login_process::table.find(state))
        .set((
            twitch_login_process::code.eq(code),
            twitch_login_process::scope.eq(scope),
        ))
        .returning(LoginProcess::as_returning())
        .get_result(connection)
        .expect("Failed to properly record successful login");
}

/// The User has not authorized the app
pub fn twitch_login_failed(state: &str, error: &str, error_description: &str) {
    let connection = &mut establish_connection();
    diesel::update(twitch_login_process::table.find(state))
        .set((
            twitch_login_process::is_broadcaster.eq(false),
            twitch_login_process::is_bot.eq(false),
        ))
        .returning(LoginProcess::as_returning())
        .get_result(connection)
        .expect("Failed to properly record failed login");
    println!("{} - {}", error, error_description); // TODO: logging?
}

/// After login with the Twitch secret handshake, save the connection details to the twitch_login_process table
pub fn save_new_access_token(
    state: &str,
    refresh_token: &str,
    token_expiry: &str,
    access_token: &str,
    token_type: &str,
) {
    let now_plus_expiry = Utc::now().timestamp() + token_expiry.parse::<i64>().unwrap();
    let connection = &mut establish_connection();
    diesel::update(twitch_login_process::table.find(state))
        .set((
            twitch_login_process::refresh_token.eq(refresh_token),
            twitch_login_process::token_expiry.eq(now_plus_expiry),
            twitch_login_process::access_token.eq(access_token),
            twitch_login_process::token_type.eq(token_type),
        ))
        .returning(LoginProcess::as_returning())
        .get_result(connection)
        .expect("Failed to save access token");
}

/// After validation, twitch hands back the user id and login, thesse are useful, so we save them
pub fn save_initial_user_details(state: &str, id: &i32, login: &str) {
    let connection = &mut establish_connection();
    // This is the example upsert, for anyone searching that term in my codebase (aka me)
    diesel::insert_into(twitch_user::table)
        .values((
            twitch_user::id.eq(id),
            twitch_user::login.eq(login),
            twitch_user::login_state.eq(state),
        ))
        // --- If there's a conflict (e.g. user already exists) update the state to associate the most recent login info
        .on_conflict(twitch_user::id)
        .do_update()
        .set(twitch_user::login_state.eq(state))
        // --- end conflict handling
        .execute(connection)
        .expect("Failed to save access token");
}

/// After validation, twitch hands back the user id and login, thesse are useful, so we save them
pub fn save_initial_bot_details(state: &str, id: &i32, login: &str, bot_owner: &i32) {
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
        .expect("Failed to save access token");
}

// Set the channel_id for the owner's channel to associate the two during the bot login process
pub fn add_bot_owner(state: &str, owner_id: &i32) {
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
        .expect("Failed to save bot access token");
}
