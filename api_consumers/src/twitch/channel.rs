//! Calls to  https://api.twitch.tv/helix/channels

use reqwest::{Error, Response};

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
