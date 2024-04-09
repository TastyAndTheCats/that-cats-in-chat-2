use reqwest::{Client, Error, Response};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct TwitchLoginSuccessResponse {
    pub code: String,
    pub scope: String,
    pub state: String,
}

#[derive(Deserialize)]
pub struct TwitchLoginFailResponse {
    pub error: String,
    pub error_description: String,
    pub state: String,
}

// Generates a random string for nonce purposes
pub fn state() -> String {
    utils::rand::generate_password(46)
}

/// Easy place to understand where these credentials are coming from
pub fn credentials_url(state: &str) -> String {
    let base_url = "https://id.twitch.tv/oauth2/authorize";
    let client_id =
        env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable.");
    let force_verify = "false";
    let redirect_uri =
        env::var("TWITCH_REDIRECT_URL").expect("Missing TWITCH_REDIRECT_URL environment variable.");
    let response_type = "code";
    let scope = "";
    utils::url::construct_url(
        base_url,
        vec![
            ("client_id", &client_id),
            ("force_verify", force_verify),
            ("redirect_uri", &redirect_uri),
            ("response_type", response_type),
            ("scope", scope),
            ("state", &state),
        ],
    )
}

/// Use the code provided to get an actual access token for the authorised Twitch account
pub async fn complete_handshake(code: &str) -> Result<Response, Error> {
    let params = vec![
        (
            "client_id",
            env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable."),
        ),
        (
            "client_secret",
            env::var("TWITCH_CLIENT_SECRET")
                .expect("Missing TWITCH_CLIENT_SECRET environment variable."),
        ),
        ("code", code.to_owned()),
        ("grant_type", "authorization_code".to_owned()),
        (
            "redirect_uri",
            env::var("TWITCH_REDIRECT_URL")
                .expect("Missing TWITCH_REDIRECT_URL environment variable."),
        ),
    ];

    let response = Client::new()
        .post("https://id.twitch.tv/oauth2/token")
        .body(utils::url::compose_post_body(params))
        .send()
        .await?;
    Ok(response)
}
