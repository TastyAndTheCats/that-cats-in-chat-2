//! Routes we use when logging in the user - and maybe shared parts too

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{prelude::*, result};

use crate::establish_connection;
use crate::models::LoginProcess;
use crate::schema::{twitch_login_process, twitch_user};

/// The admin site uses this method to login as a broadcaster, bot, or both
pub fn initiate_login(
    state: &str,
    scope: &str,
    is_broadcaster: bool,
    is_bot: bool,
) -> Result<LoginProcess, result::Error> {
    let initiated_at_dt = DateTime::from_timestamp(Utc::now().timestamp(), 0).unwrap();
    let naive_initiated_at_dt =
        NaiveDateTime::new(initiated_at_dt.date_naive(), initiated_at_dt.time());
    let login = LoginProcess {
        state: state.to_string(),
        scope: scope.to_string(),
        code: None,
        is_broadcaster: is_broadcaster,
        is_bot: is_bot,
        initiated_at: naive_initiated_at_dt, // TODO: make better
        refresh_token: None,
        token_expiry: None,
        access_token: None,
        token_type: None,
    };
    diesel::insert_into(twitch_login_process::table)
        .values(&login)
        .returning(LoginProcess::as_returning())
        .get_result(&mut establish_connection())
}

/// The User has authorized the app
pub fn twitch_login_successful(
    state: &str,
    scope: &str,
    code: &str,
) -> Result<LoginProcess, result::Error> {
    diesel::update(twitch_login_process::table.find(state))
        .set((
            twitch_login_process::code.eq(code),
            twitch_login_process::scope.eq(scope),
        ))
        .returning(LoginProcess::as_returning())
        .get_result(&mut establish_connection())
}

/// The User has not authorized the app
pub fn twitch_login_failed(
    state: &str,
    error: &str,
    error_description: &str,
) -> Result<LoginProcess, result::Error> {
    tracing::error!("{} - {}", error, error_description);
    diesel::update(twitch_login_process::table.find(state))
        .set((
            twitch_login_process::is_broadcaster.eq(false),
            twitch_login_process::is_bot.eq(false),
        ))
        .returning(LoginProcess::as_returning())
        .get_result(&mut establish_connection())
}

/// After login with the Twitch secret handshake, save the connection details to the twitch_login_process table
pub fn save_new_access_token(
    state: &str,
    refresh_token: &str,
    token_expiry: &str,
    access_token: &str,
    token_type: &str,
) -> Result<LoginProcess, result::Error> {
    let now_plus_expiry = Utc::now().timestamp() + token_expiry.parse::<i64>().unwrap();
    diesel::update(twitch_login_process::table.find(state))
        .set((
            twitch_login_process::refresh_token.eq(refresh_token),
            twitch_login_process::token_expiry.eq(now_plus_expiry),
            twitch_login_process::access_token.eq(access_token),
            twitch_login_process::token_type.eq(token_type),
        ))
        .returning(LoginProcess::as_returning())
        .get_result(&mut establish_connection())
}

/// After validation, twitch hands back the user id and login, thesse are useful, so we save them
pub fn save_initial_user_details(
    state: &str,
    id: &i32,
    login: &str,
) -> Result<usize, result::Error> {
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
        .execute(&mut establish_connection())
}

pub fn get_refresh_token(user_id: i32) -> Result<Option<String>, result::Error> {
    twitch_user::table
        .inner_join(twitch_login_process::table)
        .filter(twitch_user::id.eq(user_id))
        .select(twitch_login_process::refresh_token)
        .get_result(&mut establish_connection())
}

pub fn update_refresh_token(user_id: i32, access_token: &str, refresh_token: &str) {
    let user_state = get_user_state_from_id(user_id).unwrap_or(None);
    match user_state {
        Some(state) => {
            let state_copy = state.to_owned();
            diesel::update(twitch_login_process::table.find(state))
                .set((
                    twitch_login_process::state.eq(state_copy),
                    twitch_login_process::refresh_token.eq(refresh_token),
                    twitch_login_process::access_token.eq(access_token),
                ))
                .returning(LoginProcess::as_returning())
                .get_result(&mut establish_connection())
                .unwrap();
        }
        None => {}
    }
}

fn get_user_state_from_id(user_id: i32) -> Result<Option<String>, result::Error> {
    twitch_user::table
        .filter(twitch_user::id.eq(user_id))
        .select(twitch_user::login_state)
        .get_result::<Option<String>>(&mut establish_connection())
}
