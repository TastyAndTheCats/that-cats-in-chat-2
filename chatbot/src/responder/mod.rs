//! Contains all of the messages sent by the bot
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use types::get::channel;

use crate::{
    local_types::{faked_privmsgmessage, TwitchClient},
    responder::permissions::Permissions,
};

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
    let responder = responder.unwrap();
    println!("{:?} - {:?}", auth_level, responder);

    if auth_level == Permissions::ALL // All
        // Broadcaster-only
        || responder.requires_broadcaster && auth_level == Permissions::BROADCASTER
        // Moderator+
        || (responder.requires_moderator
            && (auth_level == Permissions::BROADCASTER || auth_level == Permissions::MODERATOR))
        // VIP+
        || (responder.requires_vip
            && (auth_level == Permissions::BROADCASTER
                || auth_level == Permissions::MODERATOR
                || auth_level == Permissions::VIP))
        // Subscriber+
        || (responder.requires_subscriber
            && (auth_level == Permissions::BROADCASTER
                || auth_level == Permissions::MODERATOR
                || auth_level == Permissions::SUBSCRIBER))
    {
        send_message_to(client, channel.login, message).await;
    } else {
        tracing::info!(
            "Insufficient permissions on call to {} from {}",
            responder.title,
            msg.sender.name
        );
    }
}

/// Run a function to generate a message to send to the TWITCH_CHANNEL
pub async fn function_message(
    responder: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();

    let message = match response_fn.as_str() {
        "" | "default" | "text" => responder.response.as_ref().unwrap().to_owned(),
        _ => {
            if response_fn.starts_with("core") {
                core::dispatch(responder, msg, command).await
            } else if response_fn.starts_with("game") {
                game::dispatch(responder, command).await
            } else {
                format!("Unknown response Function: {}", response_fn)
            }
        }
    };
    message
}
