use reqwest::{Error, Response};

use utils::{serde_json::unwrap_reqwest, twitch::client_and_access_token, url::construct_url};

use crate::twitch;

use super::users::lookup_user_from_login;

pub async fn shoutout(from_id: &str, to_login: &str) -> Result<Response, Error> {
    let (client_id, access_token) = client_and_access_token(None);
    let json_user_from_login = unwrap_reqwest(lookup_user_from_login(to_login).await).await;
    let to_id = json_user_from_login["data"][0]["id"].as_str().unwrap();
    let params = vec![
        ("from_broadcaster_id", from_id),
        ("to_broadcaster_id", &to_id),
        ("moderator_id", from_id),
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
