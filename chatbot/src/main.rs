//! The Cats In Chat is a multifunction chatbot focused around providing a wholesome experience on Twitch
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
use tokio;

mod auth;
mod definitions;
mod handler;
mod responder;

use responder::say_hello;

/// Run the chatbot stack and connect the authenticated chatbot to the TWITCH_CHANNEL provided in .env
/// NOTE: if your authentication fails it might be a permissions issue not an auth issue
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init(); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file

    let (client, incoming_messages) = auth::configure_chatbot().await;

    say_hello(&client, "HeyGuys").await;

    let join_handle = tokio::spawn(async move { handler::dispatch(incoming_messages).await });
    client
        .join(
            env::var("TWITCH_CHANNEL")
                .expect("Missing TWITCH_CHANNEL environment variable.")
                .to_ascii_lowercase()
                .to_owned(),
        )
        .unwrap();
    join_handle.await.unwrap();
}
