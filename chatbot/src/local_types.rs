//! Types which requires the libraries in this binary are defined here to keep the types lib concise

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use twitch_irc::{
    login::{RefreshingLoginCredentials, TokenStorage, UserAccessToken},
    message::{IRCMessage, IRCTags, PrivmsgMessage, TwitchUserBasics},
    transport::tcp::{TCPTransport, TLS},
    TwitchIRCClient,
};

use database::login::bot;
use types::get::{channel, chatbot};

pub type TwitchClient =
    TwitchIRCClient<TCPTransport<TLS>, RefreshingLoginCredentials<RefreshingTokenStorage>>;

#[derive(Debug)]
pub struct RefreshingTokenStorage {
    pub channel_id: i32,
}

#[async_trait]
impl TokenStorage for RefreshingTokenStorage {
    type LoadError = std::io::Error;
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        let chatbot_info = bot::bot_from_owner_id(&self.channel_id).await.unwrap();
        let login_info = bot::bot_access_token(&chatbot_info.state).await.unwrap();
        let token = login_info.access_token.unwrap();
        let refresh_token = login_info.refresh_token.unwrap();
        let created_at = login_info.initiated_at.and_utc();
        let expires_at =
            DateTime::<Utc>::from_timestamp(login_info.token_expiry.unwrap(), 0).unwrap();

        Ok(UserAccessToken {
            access_token: token,
            refresh_token: refresh_token,
            created_at: created_at,
            expires_at: Some(expires_at),
        })
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        let chatbot_info = bot::bot_from_owner_id(&self.channel_id).await.unwrap();
        let mut login_info = bot::bot_access_token(&chatbot_info.state).await.unwrap();
        let initiated_at_dt = DateTime::from_timestamp(token.created_at.timestamp(), 0).unwrap();
        let naive_initiated_at_dt =
            NaiveDateTime::new(initiated_at_dt.date_naive(), initiated_at_dt.time());

        login_info.access_token = Some(token.access_token.to_owned());
        login_info.refresh_token = Some(token.refresh_token.to_owned());
        login_info.initiated_at = naive_initiated_at_dt;
        login_info.token_expiry = Some(token.expires_at.unwrap().timestamp());

        bot::update_bot_access_token(&chatbot_info.state, login_info)
            .await
            .unwrap();

        Ok(())
    }
}

// Fakes a Privmsgmessage so we can use the send function even when not replying
// NOTE: This assumes that we'll never need moderator- or subscriber-level authentication
// when making these calls.  THe only one so far is the startup "HeyGuys" so that's fine.
pub fn faked_privmsgmessage(message: &str) -> PrivmsgMessage {
    let channel = channel(None, None);
    PrivmsgMessage {
        channel_login: channel.login,
        channel_id: channel.id.to_string(),
        message_text: message.to_owned(),
        is_action: false,
        sender: faked_twitchuserbasics(),
        badge_info: vec![],
        badges: vec![],
        bits: None,
        name_color: None,
        emotes: vec![],
        message_id: "".to_owned(),
        server_timestamp: DateTime::from_timestamp(Utc::now().timestamp(), 0).unwrap(),
        source: faked_ircmessage(),
    }
}

fn faked_twitchuserbasics() -> TwitchUserBasics {
    let bot = chatbot(None, None);
    TwitchUserBasics {
        id: bot.id.to_string(),
        login: bot.login.to_owned(),
        name: bot.name.unwrap_or(bot.login.to_owned()),
    }
}

fn faked_ircmessage() -> IRCMessage {
    IRCMessage {
        tags: faked_irctags(),
        prefix: None,
        command: "".to_owned(),
        params: vec![],
    }
}

fn faked_irctags() -> IRCTags {
    IRCTags {
        0: HashMap::from([
            ("mod".to_owned(), Some("0".to_owned())),
            ("subscriber".to_owned(), Some("0".to_owned())),
        ]),
    }
}
