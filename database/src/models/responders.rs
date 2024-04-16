//! Responders
//! They respond to messages

use crate::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::twitch_bot_responders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TwitchResponder {
    pub title: String,
    pub response: Option<String>,
    pub response_fn: Option<String>,
    pub starts_with: Option<String>,
    pub ends_with: Option<String>,
    pub contains: Option<String>,
}

impl Default for TwitchResponder {
    fn default() -> TwitchResponder {
        TwitchResponder {
            title: "".to_owned(),
            response: Some("".to_string()),
            response_fn: None,
            starts_with: None,
            ends_with: None,
            contains: None,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_selected_responders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSelectedResponder {
    pub user_id: i32,
    pub responder_id: i32,
    pub active: bool,
}
