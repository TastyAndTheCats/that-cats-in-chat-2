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
pub async fn configure_chatbot(
    id: Option<String>,
    secret: Option<String>,
    login_redirect_url: Option<String>,
    login_bot_redirect_url: Option<String>,
) -> (UnboundedReceiver<ServerMessage>, TwitchClient) {
    TwitchIRCClient::<SecureTCPTransport, RefreshingLoginCredentials<RefreshingTokenStorage>>::new(
        ClientConfig::new_simple(RefreshingLoginCredentials::init(
            app(
                id.to_owned(),
                secret.to_owned(),
                login_redirect_url.to_owned(),
                login_bot_redirect_url.to_owned(),
            )
            .client_id,
            app(
                id.to_owned(),
                secret.to_owned(),
                login_redirect_url.to_owned(),
                login_bot_redirect_url.to_owned(),
            )
            .client_secret,
            RefreshingTokenStorage {
                channel_id: channel(None, None).id,
            },
        )),
    )
}
