//! Responders
//! They respond to messages
use std::fmt;

use diesel::prelude::*;

use crate::schema;

/// The functional unit used when deciding/responding to messages
#[derive(Debug, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TwitchResponder {
    pub id: i32,
    pub responder_profile: i32,
    pub last_instance: i32,
    pub permissions: i32,
    pub cooldown: i32,
    // pub per_user_cooldown, // TODO: I need to keep track of users for this and I don't yet
    pub include_specific_users: Option<String>,
    pub exclude_specific_users: Option<String>,
    pub interval: Option<i32>,
    pub distance: Option<i32>,
    pub requires_broadcaster: bool,
    pub requires_moderator: bool,
    pub requires_vip: bool,
    pub requires_subscriber: bool,
    // pub requires_follower:bool, // TODO: this also requires I keep track of users (following is part of an api call not the msg) and I don't yet
    pub title: String,
    pub starts_with: Option<String>,
    pub contains: Option<String>,
    pub ends_with: Option<String>,
    pub response: Option<String>,
    pub response_fn: Option<String>,
}

impl Default for TwitchResponder {
    fn default() -> TwitchResponder {
        TwitchResponder {
            id: 0,
            responder_profile: 0,
            last_instance: 0,
            permissions: 0,
            cooldown: 99999,
            include_specific_users: None,
            exclude_specific_users: None,
            interval: None,
            distance: None,
            requires_broadcaster: true,
            requires_moderator: false,
            requires_subscriber: false,
            requires_vip: false,
            title: "".to_owned(),
            starts_with: None,
            contains: None,
            ends_with: None,
            response: None,
            response_fn: None,
        }
    }
}

impl fmt::Display for TwitchResponder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut permissions = String::new();
        if self.requires_broadcaster {
            permissions.push('B');
        }
        if self.requires_moderator {
            permissions.push('M');
        }
        if self.requires_subscriber {
            permissions.push('S');
        }
        if self.requires_vip {
            permissions.push('V');
        }

        let mut triggers = String::new();
        if let Some(starts_with) = &self.starts_with {
            triggers.push_str(&format!(" sw:{}", starts_with));
        }
        if let Some(contains) = &self.contains {
            triggers.push_str(&format!(" c:{}", contains));
        }
        if let Some(ends_with) = &self.ends_with {
            triggers.push_str(&format!(" ew:{}", ends_with));
        }

        let mut automatic = String::new();
        if self.interval.is_some() && self.distance.is_some() {
            automatic.push_str(&format!(
                " auto: {}/{}",
                self.interval.unwrap(),
                self.distance.unwrap()
            ));
        }

        let cooldown = format!("cd:{}/{}", self.cooldown, self.last_instance);

        write!(
            f,
            "({} {}/{} {}{}{} {})",
            self.title,
            self.response.as_ref().unwrap_or(&String::from("")),
            self.response_fn.as_ref().unwrap_or(&String::from("")),
            permissions,
            triggers,
            automatic,
            cooldown
        )
    }
}

/// The Association between a channel/User and a Responder
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::user_selected_responders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSelectedResponder {
    pub user_id: i32,
    pub responder_id: i32,
    pub responder_profile: i32,
    pub active: bool,
    pub last_instance: i32,
    pub permissions: i32,
    pub cooldown: i32,
    pub per_user_cooldown: i32,
    pub include_specific_users: Option<String>,
    pub exclude_specific_users: Option<String>,
}
