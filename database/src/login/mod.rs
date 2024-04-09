use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use super::establish_connection;
use super::models::LoginProcess;
use super::schema::twitch_login_process;

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

/// After login with the Twitch secret handshake, save the connection details to the...
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
