use std::env;
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::{
    RefreshingLoginCredentials, StaticLoginCredentials, TokenStorage, UserAccessToken,
};
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

pub async fn configure_chatbot() -> (
    TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    UnboundedReceiver<ServerMessage>,
) {
    let (incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(
            default_chatbot_config(), // static_chatbot_config()
        );

    return (client, incoming_messages);
}

fn default_chatbot_config() -> ClientConfig<StaticLoginCredentials> {
    ClientConfig::default()
}

fn static_chatbot_config() -> ClientConfig<StaticLoginCredentials> {
    let chatbot_username = env::var("CHATBOT_USERNAME")
        .expect("Missing CHATBOT_USERNAME environment variable.")
        .to_owned();
    let oauth_token = "".to_owned();

    ClientConfig::new_simple(StaticLoginCredentials::new(
        chatbot_username,
        Some(oauth_token),
    ))
}
