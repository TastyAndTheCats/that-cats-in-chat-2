//! Contains all of the messages sent by the bot
use std::env;

use crate::definitions::types::TwitchClientType;

/// Send a message to any authorized channel (this is sort of just future-proofing)
async fn send_message(client: &TwitchClientType, channel_name: String, message: &str) {
    client
        .say(channel_name, message.to_owned())
        .await
        .expect("Couldn't send message!");
}

/// Send a message to the TWITCH_CHANNEL
async fn send_default_message(client: &TwitchClientType, message: &str) {
    let channel_name =
        env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable.");
    send_message(client, channel_name, message).await;
}

// Send the classic TCIC HeyGuys startup message
pub async fn say_hello(client: &TwitchClientType) {
    send_default_message(client, "HeyGuys").await;
}

pub async fn test_command(client: &TwitchClientType) {
    send_default_message(
        client,
        "TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE",
    )
    .await;
}
