//! Handles responses to normal chat messages

use std::env;

use crate::responder;
use crate::types::TwitchClientType;
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;

mod automoderator;

/// Dispatches all of the chatbot responses. This is the main brain of the chatbot's response engine.
pub async fn dispatch(
    client: &TwitchClientType,
    msg: PrivmsgMessage,
    responders: &Vec<TwitchResponder>,
) {
    if automoderator::check(&msg.message_text) {
        for r in responders {
            if r.starts_with.is_some() {
                let options = r.starts_with.as_ref().unwrap().split("|");
                for opt in options {
                    if msg.message_text.to_lowercase().starts_with(opt) {
                        send_response_or_run_response_fn(client, &r, &msg, opt).await;
                    }
                }
            }

            if r.contains.is_some() {
                let options = r.contains.as_ref().unwrap().split("|");
                for opt in options {
                    if msg.message_text.to_lowercase().contains(opt) {
                        send_response_or_run_response_fn(client, &r, &msg, opt).await;
                    }
                }
            }

            if r.ends_with.is_some() {
                let options = r.ends_with.as_ref().unwrap().split("|");
                for opt in options {
                    if msg.message_text.to_lowercase().ends_with(opt) {
                        send_response_or_run_response_fn(client, &r, &msg, opt).await;
                    }
                }
            }
        }
    }
}



async fn send_response_or_run_response_fn(
    client: &TwitchClientType,
    r: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) {
    if r.response_fn.is_some() {
        // TODO: Find a way to run the named functions with the correct parameters?
        // Hopefully we can do all the formatting and data stuff in format_response
        // so this should just be something like
        responder::run(client, r).await;
    } else {
        let message = format_response(r, msg, command);
        responder::send(client, message).await;
    }
}

/// Formats the response message for Twitch chat
fn format_response(r: &TwitchResponder, msg: &PrivmsgMessage,
    command: &str) -> String {
    insert_data_in_response(r, msg, command)
}

/// Rwplaces text variables (no format yet) with the real data where possible
fn insert_data_in_response(r: &TwitchResponder, msg: &PrivmsgMessage,
    command: &str) -> String {
    let response = &r.response.as_ref().unwrap().to_owned();

    let response = response.replace("{sender}", &msg.sender.name);
    
    let channel_name = env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable.");
    let response = response.replace("{channel_name}", &channel_name);

    let rest_of_message = msg.message_text.split(command).collect::<Vec<&str>>()[1..].join("");
    let response = response.replace("{_1}", &rest_of_message);

    response.to_owned()
}
