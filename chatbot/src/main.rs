/// The Cats In Chat is a multifunction chatbot focussed around providing a wholesome experience on Twitch

use dotenvy::dotenv;
use tokio;
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

/// Run the chatbot stack
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init(); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file
    start_chatbot_for("tastyandthecats").await; // Start the chatbot
}

/// Easy place to understand where these credentials are coming from
fn chatbot_credentials() -> (String, String) {
    let chatbot_login = "thecatsinchat".to_owned();
    let chatbot_password = "".to_owned();
    (chatbot_login, chatbot_password)
}

/// Runs a chatbot for a given channel
/// NOTE: this will capture the program unless spawned in a thread
async fn start_chatbot_for(channel_name: &str) {
    let (
        client, 
        mut incoming_messages
    ) = configure_chatbot().await;
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    tracing::info!("Received message: {:?}", msg);
                },

                ServerMessage::Whisper(msg) => {
                    tracing::info!("Received whisper: {:?}", msg);
                },
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
    });
    client.join(channel_name.to_ascii_lowercase().to_owned()).unwrap();
    join_handle.await.unwrap();
}

async fn configure_chatbot() -> (TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>, UnboundedReceiver<ServerMessage>) {
    let config = ClientConfig::default();
    let (incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    return (client, incoming_messages);
}