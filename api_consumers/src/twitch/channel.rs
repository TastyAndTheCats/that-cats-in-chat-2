//! Calls to  https://api.twitch.tv/helix/channels

use reqwest::{Error, Response};
use utils::serde_json::unwrap_reqwest;

use crate::twitch;

pub async fn lookup_channel_from_id(channel_id: &str) -> Result<Response, Error> {
    twitch::get(
        &format!(
            " https://api.twitch.tv/helix/channels?broadcaster_id={}",
            channel_id
        ),
        None,
    )
    .await
}

pub async fn set_game(channel_id: &str, title: &str) -> Result<Response, Error> {
    let game_info = unwrap_reqwest(
        twitch::get(
            &format!("https://api.twitch.tv/helix/games?name={}", title),
            None,
        )
        .await,
    )
    .await;

    let game_id = game_info.as_object().unwrap()["data"][0]["id"]
        .as_str()
        .unwrap_or("")
        .to_owned();

    twitch::patch(
        &format!(
            " https://api.twitch.tv/helix/channels?broadcaster_id={}",
            channel_id
        ),
        Some(vec![("game_id", game_id)]),
        Some(channel_id.parse::<i32>().unwrap_or(0)),
    )
    .await
}

pub async fn set_title(channel_id: &str, title: &str) -> Result<Response, Error> {
    twitch::patch(
        &format!(
            " https://api.twitch.tv/helix/channels?broadcaster_id={}",
            channel_id
        ),
        Some(vec![("title", title.to_owned())]),
        Some(channel_id.parse::<i32>().unwrap_or(0)),
    )
    .await
}
