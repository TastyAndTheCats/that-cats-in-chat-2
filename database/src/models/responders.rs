//! Responders
//! They respond to messages

use crate::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::twitch_bot_responders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TwitchResponder {
    pub title: String,
    pub response: String,
    pub starts_with: Option<String>,
    pub ends_with: Option<String>,
    pub contains: Option<String>,
    pub requires_broadcaster: bool,
    pub requires_moderator: bool,
    pub requires_vip: bool,
    pub requires_subscriber: bool,
    pub requires_follower: bool,
}

impl Default for TwitchResponder {
    fn default() -> TwitchResponder {
        TwitchResponder {
            title: "".to_owned(),
            response: "".to_owned(),
            starts_with: None,
            ends_with: None,
            contains: None,
            requires_broadcaster: false,
            requires_moderator: false,
            requires_vip: false,
            requires_subscriber: false,
            requires_follower: false,
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
