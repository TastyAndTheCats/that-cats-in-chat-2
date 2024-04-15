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
mod handler;
mod responder;
mod types;

use database::models::responders::TwitchResponder;
use types::TwitchClientType;

/// Run the chatbot stack and connect the authenticated chatbot to the TWITCH_CHANNEL provided in .env
/// NOTE: if your authentication fails it might be a permissions issue not an auth issue
#[tokio::main]
pub async fn main() {
    start_app();
    let bot = start_bot().await;
    bot.await.unwrap();
}

/// Things needed for the app to work
fn start_app() {
    tracing_subscriber::fmt::init(); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file
}

/// Things needed for the bot to work
async fn start_bot() -> tokio::task::JoinHandle<()> {
    let (client, incoming_messages) = auth::configure_chatbot().await;
    let channel = env::var("TWITCH_CHANNEL")
        .expect("Missing TWITCH_CHANNEL environment variable.")
        .to_ascii_lowercase()
        .to_owned();

    client.join(channel).unwrap(); // NOTE: We could listen to multiple channels with the same bot, but we have to independently reply to the same channel's chat
                                   // I'm just working on getting the bot going for now, but we'll probably need to use this ability in order to scale efficiently.

    let responders = bot_initialization(&client).await;

    tokio::spawn(
        async move { handler::dispatch(&client.clone(), incoming_messages, &responders).await },
    )
}

/// Things that need to happen before the bot starts listening to the channel
async fn bot_initialization(client: &TwitchClientType) -> Vec<TwitchResponder> {
    // Load the available responders for this user
    let channel_id = utils::parse_id(
        env::var("TWITCH_CHANNEL_ID").expect("Missing TWITCH_CHANNEL_ID environment variable."),
    );

    let responders = database::bot::get_responders_list(channel_id).await;
    for r in &responders {
        println!("{:?}", r);
        match r.title.as_str() {
            "Say Hello" => {
                responder::send(client, &r.response.as_ref().unwrap()).await;
                // You cheeky little...
            }
            _ => {}
        }
    }

    responders
}
