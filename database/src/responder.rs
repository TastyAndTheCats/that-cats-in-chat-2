use chrono::Utc;
use diesel::{prelude::*, result};

use crate::establish_connection;
use crate::models::responders::UserSelectedResponder;
use crate::schema::user_selected_responders;

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
