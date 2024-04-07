use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::{StaticLoginCredentials, RefreshingLoginCredentials, TokenStorage, UserAccessToken};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use twitch_irc::message::ServerMessage;

pub async fn configure_chatbot() -> (
    TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    UnboundedReceiver<ServerMessage>,
) {
    let config = ClientConfig::default();
    let (incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    return (client, incoming_messages);
}