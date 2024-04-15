//! Contains all of the messages sent by the bot
use std::env;

use database::models::responders::TwitchResponder;

use crate::types::TwitchClientType;

/// Send a message to any authorized channel (this is sort of just future-proofing)
async fn send_message_to(client: &TwitchClientType, channel_name: String, message: &str) {
    client
        .say(channel_name, message.to_owned())
        .await
        .expect("Couldn't send message!");
}

/// Send a message to the TWITCH_CHANNEL
pub async fn send(client: &TwitchClientType, message: &str) {
    let channel_name =
        env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable.");
    send_message_to(client, channel_name, message).await;
}

/// Run a function to generate a message to send to the TWITCH_CHANNEL
pub async fn run(client: &TwitchClientType, responder: &TwitchResponder) {
    let channel_name =
        env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable.");
    let response_fn = responder.response_fn.as_ref().unwrap();
    let message = match response_fn.as_str() {
        "" | "default" | "text" => responder.response.as_ref().unwrap().to_owned(),
        "unpack_the_galaxy" => {
            unpack_the_galaxy() // TODO: remove this
        }
        _ => {
            println!("Unknown response Function: {}", response_fn);
            "".to_owned()
        }
    };
    send_message_to(client, channel_name, &message).await;
}

fn unpack_the_galaxy() -> String {
    String::from("GalaxyUnpacked GalaxyUnpacked GalaxyUnpacked GalaxyUnpacked GalaxyUnpacked")
}
