use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::env;
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::{RefreshingLoginCredentials, TokenStorage, UserAccessToken};
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use database::{bot, login};
use utils;

#[derive(Debug)]
pub struct RefreshingTokenStorage {
    channel_id: i32,
}

#[async_trait]
impl TokenStorage for RefreshingTokenStorage {
    type LoadError = std::io::Error; // or some other error
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        let chatbot_info = bot::bot_from_owner_id(&self.channel_id).await;
        let access_token = login::bot_access_token(&chatbot_info.state).await;
        let token = access_token.access_token.unwrap();
        let refresh_token = access_token.refresh_token.unwrap();
        let created_at = access_token.initiated_at.and_utc();
        let expires_at =
            DateTime::<Utc>::from_timestamp(access_token.token_expiry.unwrap(), 0).unwrap();

        Ok(UserAccessToken {
            access_token: token,
            refresh_token: refresh_token,
            created_at: created_at,
            expires_at: Some(expires_at),
        })
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        let chatbot_info = bot::bot_from_owner_id(&self.channel_id).await;
        let mut access_token = bot::bot_access_token(&chatbot_info.state).await;

        access_token.access_token = Some(token.access_token.to_owned());
        access_token.refresh_token = Some(token.refresh_token.to_owned());
        access_token.initiated_at = NaiveDateTime::from_timestamp(token.created_at.timestamp(), 0);
        access_token.token_expiry = Some(token.expires_at.unwrap().timestamp());

        database::bot::update_bot_access_token(&chatbot_info.state, access_token).await;

        Ok(())
    }
}

pub async fn configure_chatbot() -> (
    TwitchIRCClient<TCPTransport<TLS>, RefreshingLoginCredentials<RefreshingTokenStorage>>,
    UnboundedReceiver<ServerMessage>,
) {
    let channel_id = utils::parse_id(
        env::var("TWITCH_CHANNEL_ID").expect("Missing TWITCH_CHANNEL_ID environment variable."),
    );
    let config = refreshing_chatbot_config(&channel_id);
    let (incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<RefreshingTokenStorage>,
    >::new(config);

    client
        .say(env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable."), "HeyGuys".to_owned())
        .await
        .expect("Couldn't send message!");
    return (client, incoming_messages);
}

fn refreshing_chatbot_config(
    channel_id: &i32,
) -> ClientConfig<RefreshingLoginCredentials<RefreshingTokenStorage>> {
    let storage = RefreshingTokenStorage {
        channel_id: channel_id.to_owned(),
    };

    ClientConfig::new_simple(RefreshingLoginCredentials::init(
        env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable."),
        env::var("TWITCH_CLIENT_SECRET")
            .expect("Missing TWITCH_CLIENT_SECRET environment variable."),
        storage,
    ))
}
