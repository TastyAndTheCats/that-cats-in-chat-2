//! This is pretty much just how it's explained in the twitch_irc documentation.

use std::env;
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::RefreshingLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::definitions::{structs::RefreshingTokenStorage, types::TwitchClientType};
use utils;

/// Creates a refreshingLoginCredentials-based connection to Twitch
/// Returns the client (for writing messages), and the receiver (for listening to chat)
pub async fn configure_chatbot() -> (TwitchClientType, UnboundedReceiver<ServerMessage>) {
    let channel_id = utils::parse_id(
        env::var("TWITCH_CHANNEL_ID").expect("Missing TWITCH_CHANNEL_ID environment variable."),
    );

    let storage = RefreshingTokenStorage {
        channel_id: channel_id,
    };
    let client_id =
        env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable.");
    let client_secret = env::var("TWITCH_CLIENT_SECRET")
        .expect("Missing TWITCH_CLIENT_SECRET environment variable.");

    let config = ClientConfig::new_simple(RefreshingLoginCredentials::init(
        client_id,
        client_secret,
        storage,
    ));
    let (incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<RefreshingTokenStorage>,
    >::new(config);

    return (client, incoming_messages);
}
