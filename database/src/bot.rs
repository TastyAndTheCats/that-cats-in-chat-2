use diesel::prelude::*;

use crate::establish_connection;
use crate::models::{LoginProcess, TwitchBot};
use crate::schema::{twitch_bot, twitch_login_process};

/// Takes the channel_id and returns the TwitchBot object associated with it
pub async fn bot_from_owner_id(owner_id: &i32) -> TwitchBot {
    let connection = &mut establish_connection();
    let bot = twitch_bot::table
        .filter(twitch_bot::channel_id.eq(owner_id))
        .select(TwitchBot::as_select())
        .get_result(connection)
        .unwrap();
    bot
}

/// Takes the login state and returns the associated LoginProcess
pub async fn bot_access_token(state: &str) -> LoginProcess {
    let connection = &mut establish_connection();
    let login_info = twitch_login_process::table
        .filter(twitch_login_process::state.eq(state))
        .select(LoginProcess::as_select())
        .get_result(connection)
        .unwrap();
    // TODO?: Check token validity
    login_info
}

/// Updates the LoginProcess for a given Login state
pub async fn update_bot_access_token(state: &str, new_login_process: LoginProcess) -> LoginProcess {
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
        .unwrap()
}
