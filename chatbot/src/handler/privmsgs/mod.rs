//! Handles responses to normal chat messages

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
            if r.starts_with.is_some()
                && msg
                    .message_text
                    .starts_with(&*r.starts_with.as_ref().unwrap())
                || r.ends_with.is_some()
                    && msg.message_text.ends_with(&*r.ends_with.as_ref().unwrap())
                || r.contains.is_some() && msg.message_text.contains(&*r.contains.as_ref().unwrap())
            {
                send_response_or_run_response_fn(client, r).await;
            }
        }
    }
}

async fn send_response_or_run_response_fn(client: &TwitchClientType, r: &TwitchResponder) {
    if r.response_fn.is_some() {
        // TODO: Find a way to run the named functions with the correct parameters?
        // Hopefully we can do all the formatting and data stuff in format_response
        // so this should just be something like
        responder::run(client, &r).await;
    } else {
        let message = format_response(&r.response.as_ref().unwrap());
        responder::send(client, message).await;
    }
}

/// Formats the response message for Twitch chat
fn format_response(message: &str) -> &str {
    let formatted_message = message;
    insert_data_in_response(formatted_message)
}

/// Rwplaces text variables (no format yet) with the real data where possible
fn insert_data_in_response(message: &str) -> &str {
    message
}
