//! Calls to https://api.twitch.tv/helix/users

use reqwest::{Client, Response};
use std::env;

use database::twitch_access_token;
use utils;

pub async fn lookup_user_from_login(login: &str) -> Response {
    let client_id =
        env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable.");
    let access_token = twitch_access_token().unwrap();
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
