use database::bot::bot_from_owner_id;
use reqwest::{Error, Response};

use utils::{serde_json::unwrap_reqwest, twitch::client_and_access_token, url::construct_url};

use crate::twitch;

use super::users::lookup_user_from_login;

// TODO: these should be the bot making this call not the user, I need to fix the scopes and fix the fn inputs

pub async fn shoutout(from_id: &str, to_login: &str) -> Result<Response, Error> {
    let from_id = from_id;
    let (client_id, access_token) = client_and_access_token();
    let bot = bot_from_owner_id(&from_id.parse::<i32>().unwrap_or(0)).unwrap();
    let bot_id = format!("{}", bot.id.unwrap_or(from_id.parse::<i32>().unwrap_or(0)));
    let json_user_from_login = unwrap_reqwest(lookup_user_from_login(to_login).await).await;
    let to_id = json_user_from_login["data"][0]["id"].as_str().unwrap();
    let params: Vec<(&str, &str)> = vec![
        ("from_broadcaster_id", from_id),
        ("to_broadcaster_id", to_id),
        ("moderator_id", &bot_id),
    ];
    let access_token = access_token.expect("Invalid client_id");

    let url = construct_url("https://api.twitch.tv/helix/chat/shoutouts", params);

    tracing::debug!(
        "url: {}, Bearer: {}, Client-Id: {}",
        url,
        access_token,
        client_id
    );

    twitch::post(&url, None, Some(from_id.parse::<i32>().unwrap())).await
}

pub async fn ban_user(from_id: &str, to_login: &str) -> Result<Response, Error> {
    let (client_id, access_token) = client_and_access_token();
    let json_user_from_login = unwrap_reqwest(lookup_user_from_login(to_login).await).await;
    let to_id = json_user_from_login["data"][0]["id"].as_str().unwrap();
    let params = vec![("broadcaster_id", from_id), ("moderator_id", from_id)];
    let access_token = access_token.expect("Invalid client_id");

    let url = construct_url("https://api.twitch.tv/helix/moderation/bans", params);

    tracing::debug!(
        "url: {}, Bearer: {}, Client-Id: {}",
        url,
        access_token,
        client_id
    );

    let body_params = vec![("data", format!("{{\"data\": {{\"user_id\":\"{}\",\"duration\":1,\"reason\":\"TheCatsInChat automoderated this\"}}}}", to_id))];

    twitch::post(
        &url,
        Some(body_params),
        Some(from_id.parse::<i32>().unwrap()),
    )
    .await
}
