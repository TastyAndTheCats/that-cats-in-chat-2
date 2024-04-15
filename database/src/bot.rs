//! Database routes related to the chatbot's state

use diesel::prelude::*;

use crate::establish_connection;
use crate::models::responders::TwitchResponder;
use crate::schema::{twitch_bot_responders, user_selected_responders};

pub async fn get_responders_list(user_id: i32) -> Vec<TwitchResponder> {
    println!("Checking for responders list (hmm)");
    let connection = &mut establish_connection();
    let responders = twitch_bot_responders::table
        .inner_join(user_selected_responders::table)
        .filter(user_selected_responders::user_id.eq(user_id))
        .select(TwitchResponder::as_select())
        .load(connection)
        .expect("Error loading responders");
    responders
}
