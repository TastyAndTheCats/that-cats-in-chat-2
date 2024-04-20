//! This is pretty much just how it's explained in the twitch_irc documentation.

use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::RefreshingLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::local_types::{RefreshingTokenStorage, TwitchClient};
use types::get::{app, channel};

/// Creates a refreshingLoginCredentials-based connection to Twitch
/// Returns the client (for writing messages), and the receiver (for listening to chat)
pub async fn configure_chatbot() -> (TwitchClient, UnboundedReceiver<ServerMessage>) {
    let channel = channel(None, None);
    let app = app(None, None, None, None);

    let config = ClientConfig::new_simple(RefreshingLoginCredentials::init(
        app.client_id,
        app.client_secret,
        RefreshingTokenStorage {
            channel_id: channel.id,
        },
    ));
    let (incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<RefreshingTokenStorage>,
    >::new(config);

    return (client, incoming_messages);
}
