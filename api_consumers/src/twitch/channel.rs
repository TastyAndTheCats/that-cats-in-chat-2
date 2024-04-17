//! Calls to  https://api.twitch.tv/helix/channels

use reqwest::{Client, Response};

use utils::twitch::client_and_access_token;

pub async fn lookup_channel_from_id(channel_id: &str) -> Response {
    let (client_id, access_token) = client_and_access_token();

    Client::new()
        .get(format!(
            " https://api.twitch.tv/helix/channels?broadcaster_id={}",
            channel_id
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", client_id)
        .send()
        .await
        .unwrap()
}
