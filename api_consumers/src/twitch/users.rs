//! Calls to https://api.twitch.tv/helix/users
use reqwest::{Error, Response};

use crate::twitch::get;

pub async fn lookup_user_from_login(login: &str) -> Result<Response, Error> {
    get(
        &format!(
            "https://api.twitch.tv/helix/users?login={}",
            login.to_lowercase()
        ),
        None,
    )
    .await
}
