//! Contains all of the messages sent by the bot
use twitch_irc::message::{PrivmsgMessage, TwitchUserBasics};

use database::models::responders::TwitchResponder;
use types::get::{channel, chatbot};

use crate::local_types::{faked_privmsgmessage, TwitchClient};

mod core;
mod game;
pub mod permissions;

/// Send a message to any authorized channel (this is sort of just future-proofing)
async fn send_message_to(client: &TwitchClient, channel_name: String, message: String) {
    client
        .me(channel_name, message.replace('\n', "").to_owned())
        .await
        .expect("Couldn't send message!");
}

/// Send a message to the TWITCH_CHANNEL
pub async fn send(
    client: &TwitchClient,
    msg: Option<&PrivmsgMessage>,
    message: String,
    responder: Option<&TwitchResponder>,
) {
    let channel = channel(None, None);
    let privmsg = faked_privmsgmessage(&message);
    let msg = msg.unwrap_or(&privmsg);

    let auth_level = permissions::check(msg);
    println!("{:?} - {:?}", auth_level, responder);

    send_message_to(client, channel.login, message).await;
}

/// Run a function to generate a message to send to the TWITCH_CHANNEL
pub async fn function_message(
    client: &TwitchClient,
    responder: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();

    let message = match response_fn.as_str() {
        "" | "default" | "text" => responder.response.as_ref().unwrap().to_owned(),
        _ => {
            if response_fn.starts_with("core") {
                core::dispatch(client, responder, msg, command).await
            } else if response_fn.starts_with("game") {
                game::dispatch(responder, command).await
            } else {
                format!("Unknown response Function: {}", response_fn)
            }
        }
    };
    message
}
