//! Responders
//! They respond to messages

use crate::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::twitch_bot_responders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TwitchResponder {
    title: String,
    response: String,
    starts_with: Option<String>,
    ends_with: Option<String>,
    contains: Option<String>,
    requires_broadcaster: Option<bool>,
    requires_moderator: Option<bool>,
    requires_vip: Option<bool>,
    requires_subscriber: Option<bool>,
    requires_follower: Option<bool>,
}

impl Default for TwitchResponder {
    fn default() -> TwitchResponder {
        TwitchResponder {
            title: "".to_owned(),
            response: "".to_owned(),
            starts_with: None,
            ends_with: None,
            contains: None,
            requires_broadcaster: Some(false),
            requires_moderator: Some(false),
            requires_vip: Some(false),
            requires_subscriber: Some(false),
            requires_follower: Some(false),
        }
    }
}
