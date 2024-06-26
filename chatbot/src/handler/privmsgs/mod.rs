//! Handles responses to normal chat messages
use database::channel::increment_messages_counted;
use regex::Regex;
use std::env;

use crate::local_types::{faked_privmsgmessage, TwitchClient};
use crate::responder;
use api_consumers::twitch::users::lookup_user_from_login;
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::message::rest_of_chat_message;
use utils::serde_json::unwrap_reqwest;

mod automoderator;

/// Dispatches all of the chatbot responses. This is the main brain of the chatbot's response engine.
pub async fn dispatch(client: TwitchClient, msg: PrivmsgMessage, responders: Vec<TwitchResponder>) {
    if automoderator::check(&msg.message_text) {
        let _ = increment_messages_counted(msg.channel_id.parse::<i32>().unwrap_or(0));
        let m = msg.message_text.to_lowercase();
        for r in responders {
            if let Some(starts_with) = &r.starts_with {
                dispatch_with_regex(
                    client.clone(),
                    msg.clone(),
                    &r,
                    m.to_owned(),
                    starts_with,
                    (r"^", r"\b.*?$"),
                );
            }
            if let Some(contains) = &r.contains {
                dispatch_with_regex(
                    client.clone(),
                    msg.clone(),
                    &r,
                    m.to_owned(),
                    contains,
                    (r"^.*\W?", r"\b.*?$"),
                );
            }
            if let Some(ends_with) = &r.ends_with {
                dispatch_with_regex(
                    client.clone(),
                    msg.clone(),
                    &r,
                    m.to_owned(),
                    ends_with,
                    (r"^.*\W?", r"$"),
                );
            }
        }
    }
}

fn dispatch_with_regex(
    client: TwitchClient,
    msg: PrivmsgMessage,
    r: &TwitchResponder,
    m: String,
    parts_list: &String,
    re_template: (&str, &str),
) {
    let string_options: Vec<&str> = parts_list.split("|").collect();
    let regex_options: Vec<(String, Regex)> = string_options
        .into_iter()
        .map(|cmd| {
            (
                cmd.to_owned(),
                Regex::new(&format!(r"{}{}{}", re_template.0, cmd, re_template.1)).unwrap(),
            )
        })
        .rev()
        .collect();

    for (opt, re) in regex_options {
        let re = re.to_owned(); // Convert opt to owned String
        let r = r.clone();
        if re.is_match(&m) {
            tokio::spawn(async move {
                send_response_or_run_response_fn(client, r, Some(msg), Some(&opt)).await;
            });
            break;
        }
    }
}

pub async fn send_response_or_run_response_fn(
    client: TwitchClient,
    r: TwitchResponder,
    msg: Option<PrivmsgMessage>,
    command: Option<&str>,
) {
    let msg = msg.unwrap_or(faked_privmsgmessage(
        &r.clone().show_command_as.unwrap_or(String::new()),
    ));
    let command = command.unwrap_or("");
    responder::send(
        client,
        Some(&msg),
        match r.response_fn.is_some() {
            true => {
                insert_data_in_response(
                    responder::function_message(r.clone(), msg.clone(), command).await,
                    msg.clone(),
                    command,
                )
                .await
            }
            false => {
                format_response(
                    &r.response.as_ref().unwrap().to_owned(),
                    msg.clone(),
                    command,
                )
                .await
            }
        },
        Some(&r),
    )
    .await;
}

/// Formats the response message for Twitch chat
async fn format_response(r: &String, msg: PrivmsgMessage, command: &str) -> String {
    insert_data_in_response(r.to_owned(), msg, command).await
}

/// Rwplaces text variables (no format yet) with the real data where possible
async fn insert_data_in_response(response: String, msg: PrivmsgMessage, command: &str) -> String {
    let mut response = replace_sender_name(response, msg.clone());
    response = replace_channel_name(response, msg.clone()).await;
    replace_1_(response, msg.clone(), command)
}

/// Replace {sender} with message sender display name
fn replace_sender_name(response: String, msg: PrivmsgMessage) -> String {
    if response.contains("{sender}") {
        response.replace("{sender}", &msg.sender.name)
    } else {
        response
    }
}

/// Replace {channel_name} with channel display name
async fn replace_channel_name(response: String, msg: PrivmsgMessage) -> String {
    if response.contains("{channel_name}") {
        let user = unwrap_reqwest(lookup_user_from_login(&msg.channel_login).await).await;
        tracing::debug!("replace_channel_name: {}", user);
        response.replace(
            "{channel_name}",
            user["data"][0]["display_name"]
                .as_str()
                .unwrap_or(&env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL is missing")),
        )
    } else {
        response
    }
}

/// replace {_1}
/// NOTE: probably don't do this? it's for old commands that I don't want to update for whatever reason
fn replace_1_(response: String, msg: PrivmsgMessage, command: &str) -> String {
    if response.contains("{_1}") {
        let rest_of_message = rest_of_chat_message(&msg, command);
        response.replace("{_1}", &rest_of_message)
    } else {
        response
    }
}
