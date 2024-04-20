use diesel::{prelude::*, result};
use types::get::chatbot;

use crate::{
    establish_connection,
    models::{responders::TwitchResponder, LoginProcess},
    schema::{twitch_bot_responders, twitch_login_process, twitch_user, user_selected_responders},
};

pub async fn get_responders_for_user(user_id: i32) -> Result<Vec<TwitchResponder>, result::Error> {
    let connection = &mut establish_connection();
    twitch_bot_responders::table
        .inner_join(user_selected_responders::table)
        .filter(user_selected_responders::user_id.eq(user_id))
        .select(TwitchResponder::as_select())
        .load(connection)
}

pub fn get_access_token(id: Option<i32>) -> Result<LoginProcess, result::Error> {
    let connection = &mut establish_connection();
    let id = id.unwrap_or(chatbot(None, None).id);
    twitch_user::table
        .inner_join(twitch_login_process::table)
        .filter(twitch_user::id.eq(id))
        .select(LoginProcess::as_select())
        .get_result(connection)
}
