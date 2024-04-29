//! Models for using the Diesel schema as objects
//! The objects are sort of self documening as to where they're from and what they're representing, so the comments here are minimal
//!

use crate::schema;
use diesel::prelude::*;

pub mod responders;

// #[derive(Queryable, Selectable, Debug)]
// #[diesel(table_name = crate::schema::twitch_channel)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct TwitchChannel {
//     pub id: String,
//     pub login: String,
//     pub name: String,
//     pub language: String,
//     pub delay: i32,
// }

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = schema::twitch_login_process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoginProcess {
    pub state: String,
    pub scope: String,
    pub code: Option<String>, // If code exists, permission was given to login as user, otherwise no
    pub is_bot: bool,
    pub is_broadcaster: bool,
    pub initiated_at: chrono::NaiveDateTime,
    pub refresh_token: Option<String>,
    pub token_expiry: Option<i64>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::twitch_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TwitchUser {
    pub id: i32,
    pub login: String,
    pub login_state: Option<String>,
    pub messages_processed: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::twitch_bot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TwitchBot {
    pub state: String,
    pub id: Option<i32>,
    pub login: Option<String>,
    pub channel_id: i32,
}
