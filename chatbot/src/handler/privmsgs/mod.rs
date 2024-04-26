//! Handles responses to normal chat messages
use std::env;

use crate::local_types::TwitchClient;
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
        let m = msg.message_text.to_lowercase();
        for r in responders {
            if let Some(starts_with) = &r.starts_with {
                let starts_with = starts_with.clone(); // Clone starts_with
                for opt in starts_with.split("|") {
                    let responder = r.clone();
                    let this_client = client.clone();
                    let this_msg = msg.clone();
                    let opt = opt.to_owned(); // Convert opt to owned String
                    if m.starts_with(&opt) {
                        tokio::spawn(async move {
                            send_response_or_run_response_fn(
                                this_client,
                                responder,
                                this_msg,
                                &opt,
                            )
                            .await;
                        });
                    }
                }
            }

            if let Some(contains) = &r.contains {
                let contains = contains.clone(); // Clone contains
                for opt in contains.split("|") {
                    let responder = r.clone();
                    let this_client = client.clone();
                    let this_msg = msg.clone();
                    let opt = opt.to_owned(); // Convert opt to owned String
                    if m.contains(&opt) {
                        tokio::spawn(async move {
                            send_response_or_run_response_fn(
                                this_client,
                                responder,
                                this_msg,
                                &opt,
                            )
                            .await;
                        });
                    }
                }
            }

            if let Some(ends_with) = &r.ends_with {
                let ends_with = ends_with.clone(); // Clone ends_with
                for opt in ends_with.split("|") {
                    let responder = r.clone();
                    let this_client = client.clone();
                    let this_msg = msg.clone();
                    let opt = opt.to_owned(); // Convert opt to owned String
                    if m.ends_with(&opt) {
                        tokio::spawn(async move {
                            send_response_or_run_response_fn(
                                this_client,
                                responder,
                                this_msg,
                                &opt,
                            )
                            .await;
                        });
                    }
                }
            }
        }
    }
}

async fn send_response_or_run_response_fn(
    client: TwitchClient,
    r: TwitchResponder,
    msg: PrivmsgMessage,
    command: &str,
) {
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
