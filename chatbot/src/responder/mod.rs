use std::env;

use crate::definitions::types::TwitchClientType;

pub async fn send_message(client: &TwitchClientType, message: &str) {
    let channel_name =
        env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable.");
    client
        .say(channel_name, message.to_owned())
        .await
        .expect("Couldn't send message!");
}
