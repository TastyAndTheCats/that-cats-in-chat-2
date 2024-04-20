//! Contains all of the messages sent by the bot
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use types::get::channel;

use crate::local_types::TwitchClient;

mod core;
mod game;

/// Send a message to any authorized channel (this is sort of just future-proofing)
async fn send_message_to(client: &TwitchClient, channel_name: String, message: String) {
    client
        .me(channel_name, message.replace('\n', "").to_owned())
        .await
        .expect("Couldn't send message!");
}

/// Send a message to the TWITCH_CHANNEL
pub async fn send(client: &TwitchClient, message: String) {
    let channel_login = channel(None, None).login;
    send_message_to(client, channel_login, message).await;
}

/// Run a function to generate a message to send to the TWITCH_CHANNEL
pub async fn run(
    client: &TwitchClient,
    responder: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) {
    let channel_login = msg.channel_login.to_string();
    let response_fn = responder.response_fn.as_ref().unwrap();

    let message = match response_fn.as_str() {
        "" | "default" | "text" => responder.response.as_ref().unwrap().to_owned(),
        _ => {
            if response_fn.starts_with("core") {
                core::dispatch(client, responder, msg, command).await
            } else if response_fn.starts_with("game") {
                game::dispatch(client, responder, msg, command).await
            } else {
                format!("Unknown response Function: {}", response_fn)
            }
        }
    };
    send_message_to(client, channel_login, message).await;
}
