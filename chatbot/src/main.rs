//! The Cats In Chat is a multifunction chatbot focused around providing a wholesome experience on Twitch
//!
//! The first TCIC was written in Python and started as apersonal project that ended up getting way bigger than intended.
//! TCIC2 is a fresh start, open-sourced and written in Rust with many more expected moving parts than the original.
//!
//! I'm just getting started with this, so there aren't really any features and some stuff might end up being weirdly hard coded for now
//! I will do my best to eliminate any need for an end user to change any code, and once that's done I'll start making executable releases, maybe?
//!
//! I also hope to revive the old method of spawning a chatbot instance from a website, allowing custom commands, etc. but that's a bit down the line.
#![warn(missing_debug_implementations)]

use dotenvy::dotenv;
use tokio;

mod auth;
mod handler;
mod local_types;
mod responder;

use database::models::responders::TwitchResponder;
use local_types::TwitchClient;
use types::get::channel;

/// Run the chatbot stack and connect the authenticated chatbot to the TWITCH_CHANNEL provided in .env
/// NOTE: if your authentication fails it might be a permissions issue not an auth issue
#[tokio::main]
pub async fn main() {
    start_app();
    start_bot().await.await.expect("Chatbot has died!");
}

/// Things needed for the app to work
fn start_app() {
    tracing_subscriber::fmt::init(); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file
}

/// Things needed for the bot to work
async fn start_bot() -> tokio::task::JoinHandle<()> {
    let (incoming_messages, client) = auth::configure_chatbot(None, None, None, None).await;
    let responders = bot_initialization().await;

    client.join(channel(None, None).login).unwrap(); // NOTE: We could listen to multiple channels with the same bot, but we have to independently reply to the same channel's chat
    list_responders_in_console(&responders);
    say_hello(&client, &responders).await;

    tokio::spawn(
        async move { handler::dispatch(&client.clone(), incoming_messages, &responders).await },
    )
}

/// Things that need to happen before the bot starts listening to the channel
async fn bot_initialization() -> Vec<TwitchResponder> {
    // Load the available responders for this user
    database::bot::get_combined_responders_for_user(channel(None, None).id).unwrap_or(Vec::from([]))
}

/// Hard-coded pre-init greeting
async fn say_hello(client: &TwitchClient, responders: &Vec<TwitchResponder>) {
    for r in responders {
        match r.title.as_str() {
            "Say Hello" => {
                responder::send(
                    client,
                    None,
                    r.response.as_ref().unwrap().to_string(),
                    Some(r),
                )
                .await;
                // You cheeky little...
            }
            _ => {}
        }
    }
}

/// TODO: remove this, it's nice to be able to see what's loaded though
fn list_responders_in_console(responders: &Vec<TwitchResponder>) {
    for r in responders {
        println!("{}", r);
    }
}
