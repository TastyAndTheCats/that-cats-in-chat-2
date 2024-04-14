//! The Cats In Chat is a multifunction chatbot focussed around providing a wholesome experience on Twitch
//! 
//! The first TCIC was written in Python and started as apersonal project that ended up getting way bigger than intended.
//! TCIC2 is a fresh start, open-sourced and written in Rust with many more expected moving parts than the original.
//! 
//! I'm just getting started with this, so there aren't really any features and some stuff might end up being weirdly hard coded for now
//! I will do my best to eliminate any need for an end user to change any code, and once that's done I'll start making executable releases, maybe?
//! 
//! I also hope to revive the old method of spawning a chatbot instance from a website, allowing custom commands, etc. but that's a bit down the line.

use dotenvy::dotenv;
use std::env;
use tokio::{self, sync::mpsc::UnboundedReceiver};
use twitch_irc::message::ServerMessage;

mod auth;

/// Run the chatbot stack
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init(); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file
    start_chatbot_for(
        &env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable."),
    )
    .await; // Start the chatbot
}

/// Runs a chatbot for a given channel
async fn start_chatbot_for(channel_name: &str) {
    let (client, incoming_messages) = auth::configure_chatbot().await;
    let join_handle = tokio::spawn(async move { handle_twitch_message(incoming_messages).await });
    client
        .join(channel_name.to_ascii_lowercase().to_owned())
        .unwrap();
    join_handle.await.unwrap();
}

async fn handle_twitch_message(mut incoming_messages: UnboundedReceiver<ServerMessage>) {
    while let Some(message) = incoming_messages.recv().await {
        match message {
            ServerMessage::Privmsg(msg) => {
                tracing::info!(
                    "#{} {}: {}",
                    msg.channel_login,
                    msg.sender.name,
                    msg.message_text
                );
            }

            ServerMessage::Whisper(msg) => {
                tracing::info!("Received whisper: {:?}", msg);
            }
            ServerMessage::Ping(msg) => {
                tracing::debug!("Pinged: {:?}", msg);
            }
            ServerMessage::Pong(msg) => {
                tracing::debug!("Ponged: {:?}", msg);
            }
            _ => {
                tracing::info!("Received something else: {:?}", message);
            }
        }
    }
}
