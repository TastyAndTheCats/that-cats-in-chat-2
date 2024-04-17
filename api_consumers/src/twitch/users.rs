//! Calls to https://api.twitch.tv/helix/users

use reqwest::{Client, Response};

use utils::{self, twitch::client_and_access_token};

pub async fn lookup_user_from_login(login: &str) -> Response {
    let (client_id, access_token) = client_and_access_token();
    Client::new()
        .get(format!("https://api.twitch.tv/helix/users?login={}", login))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", client_id)
        .body(utils::url::compose_post_body(vec![(
            "login",
            login.to_lowercase(),
        )]))
        .send()
        .await
        .unwrap()
}
