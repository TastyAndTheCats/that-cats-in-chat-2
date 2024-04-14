use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use twitch_irc::login::{TokenStorage, UserAccessToken};

use database::{bot, login};

#[derive(Debug)]
pub struct RefreshingTokenStorage {
    pub channel_id: i32,
}

#[async_trait]
impl TokenStorage for RefreshingTokenStorage {
    type LoadError = std::io::Error;
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        let chatbot_info = bot::bot_from_owner_id(&self.channel_id).await;
        let login_info = login::bot_access_token(&chatbot_info.state).await;
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
        let chatbot_info = bot::bot_from_owner_id(&self.channel_id).await;
        let mut login_info = bot::bot_access_token(&chatbot_info.state).await;

        login_info.access_token = Some(token.access_token.to_owned());
        login_info.refresh_token = Some(token.refresh_token.to_owned());
        login_info.initiated_at = NaiveDateTime::from_timestamp(token.created_at.timestamp(), 0);
        login_info.token_expiry = Some(token.expires_at.unwrap().timestamp());

        bot::update_bot_access_token(&chatbot_info.state, login_info).await;

        Ok(())
    }
}