use chrono::Utc;
use diesel::{prelude::*, result};

use crate::establish_connection;
use crate::models::responders::UserSelectedResponder;
use crate::schema::{twitch_bot_responders, user_selected_responders};

pub fn update_last_instance(
    user_id: i32,
    responder_id: i32,
) -> Result<UserSelectedResponder, result::Error> {
    diesel::update(
        user_selected_responders::table
            .filter(user_selected_responders::user_id.eq(user_id))
            .filter(user_selected_responders::responder_id.eq(responder_id)),
    )
    .set(
        user_selected_responders::last_instance
            .eq(i32::try_from(Utc::now().timestamp()).expect("good until 2038")),
    )
    .returning(UserSelectedResponder::as_returning())
    .get_result(&mut establish_connection())
}

pub fn get_last_instance(user_id: i32, responder_id: i32) -> Result<i32, result::Error> {
    user_selected_responders::table
        .filter(user_selected_responders::user_id.eq(user_id))
        .filter(user_selected_responders::responder_id.eq(responder_id))
        .select(user_selected_responders::last_instance)
        .get_result(&mut establish_connection())
}

pub fn update_last_automatic_instance(
    user_id: i32,
    responder_id: i32,
) -> Result<UserSelectedResponder, result::Error> {
    diesel::update(
        user_selected_responders::table
            .filter(user_selected_responders::user_id.eq(user_id))
            .filter(user_selected_responders::responder_id.eq(responder_id)),
    )
    .set(
        user_selected_responders::last_automatic_instance
            .eq(i32::try_from(Utc::now().timestamp()).expect("good until 2038")),
    )
    .returning(UserSelectedResponder::as_returning())
    .get_result(&mut establish_connection())
}

pub fn get_last_automatic_instance(user_id: i32, responder_id: i32) -> Result<i32, result::Error> {
    user_selected_responders::table
        .filter(user_selected_responders::user_id.eq(user_id))
        .filter(user_selected_responders::responder_id.eq(responder_id))
        .select(user_selected_responders::last_automatic_instance)
        .get_result(&mut establish_connection())
}

pub fn update_count(
    user_id: i32,
    responder_id: i32,
    count: i32,
) -> Result<UserSelectedResponder, result::Error> {
    diesel::update(
        user_selected_responders::table
            .filter(user_selected_responders::user_id.eq(user_id))
            .filter(user_selected_responders::responder_id.eq(responder_id)),
    )
    .set(user_selected_responders::count.eq(count))
    .returning(UserSelectedResponder::as_returning())
    .get_result(&mut establish_connection())
}

pub fn get_count(user_id: i32, responder_id: i32) -> Result<i32, result::Error> {
    user_selected_responders::table
        .filter(user_selected_responders::user_id.eq(user_id))
        .filter(user_selected_responders::responder_id.eq(responder_id))
        .select(user_selected_responders::count)
        .get_result(&mut establish_connection())
}

pub fn update_last_automatic_message_count(
    user_id: i32,
    responder_id: i32,
    count: i32,
) -> Result<UserSelectedResponder, result::Error> {
    diesel::update(
        user_selected_responders::table
            .filter(user_selected_responders::user_id.eq(user_id))
            .filter(user_selected_responders::responder_id.eq(responder_id)),
    )
    .set(user_selected_responders::message_count_at_last_automatic.eq(count))
    .returning(UserSelectedResponder::as_returning())
    .get_result(&mut establish_connection())
}

pub fn get_last_automatic_message_count(
    user_id: i32,
    responder_id: i32,
) -> Result<i32, result::Error> {
    user_selected_responders::table
        .filter(user_selected_responders::user_id.eq(user_id))
        .filter(user_selected_responders::responder_id.eq(responder_id))
        .select(user_selected_responders::message_count_at_last_automatic)
        .get_result(&mut establish_connection())
}

pub fn get_user_responders_with_display_names(
    user_id: i32,
) -> Result<Vec<Option<String>>, result::Error> {
    user_selected_responders::table
        .inner_join(twitch_bot_responders::table)
        .filter(user_selected_responders::user_id.eq(user_id))
        .filter(twitch_bot_responders::active.eq(true))
        .filter(user_selected_responders::active.eq(true))
        .select(twitch_bot_responders::show_command_as)
        .get_results(&mut establish_connection())
}
