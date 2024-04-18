//! API Routes for Logging Into Twitch

use reqwest::{Client, Error, Response};
use serde::Deserialize;
use std::env;

/// Successful Login response
#[derive(Deserialize)]
pub struct TwitchLoginSuccessResponse {
    pub code: String,
    pub scope: String,
    pub state: String,
}

/// Failed Login struct
#[derive(Deserialize)]
pub struct TwitchLoginFailResponse {
    pub error: String,
    pub error_description: String,
    pub state: String,
}

// UNUSED: Allows the Client Credential flow, I just ended up not using it
// pub struct TwitchClientCredentials{
//     access_token: String,
//     expires_in: i32,
//     token_type: String,
// }

/// Generates a random string for nonce purposes
pub fn state() -> String {
    utils::rand::generate_password(46)
}

/// Easy place to understand where these credentials are coming from
pub fn credentials_url(state: &str, scope: &str, redirect_uri: &str) -> String {
    let base_url = "https://id.twitch.tv/oauth2/authorize";
    let client_id =
        env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable.");
    let force_verify = "false";
    let response_type = "code";
    utils::url::construct_url(
        base_url,
        vec![
            ("client_id", &client_id),
            ("force_verify", force_verify),
            ("redirect_uri", redirect_uri),
            ("response_type", response_type),
            ("scope", scope),
            ("state", state),
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

/// This validates the access_token AND gives you the user's login and id
pub async fn validate_access_token(access_token: &str) -> Result<Response, Error> {
    let response = Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("OAuth {}", access_token))
        .send()
        .await?;
    Ok(response)
}

// UNUSED: Allows the Client Credential flow, I just ended up not using it
// pub async fn client_credentials() -> TwitchClientCredentials {
//     let params = vec![
//         (
//             "client_id",
//             env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable."),
//         ),
//         (
//             "client_secret",
//             env::var("TWITCH_CLIENT_SECRET")
//                 .expect("Missing TWITCH_CLIENT_SECRET environment variable."),
//         ),
//         ("grant_type", "client_credentials".to_owned()),
//     ];

//     let response = Client::new()
//         .post("https://id.twitch.tv/oauth2/token")
//         .body(utils::url::compose_post_body(params))
//         .send()
//         .await.unwrap();

//         let client_credentials  =  serde_json::from_str(response).unwrap();
//         client_credentials

// }

/// Checks if the login is valid

pub async fn get_userid_and_login_from_validated_access_token(
    access_token: &str,
) -> [Option<String>; 2] {
    let validation_json: serde_json::Value = serde_json::from_str(
        &validate_access_token(&access_token)
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    )
    .unwrap();
    if validation_json["status"] == 401 {
        invalidate_login();
        return [None, None];
    }
    return [
        Some(validation_json["user_id"].as_str().unwrap().to_owned()),
        Some(validation_json["login"].as_str().unwrap().to_owned()),
    ];
}

/// TODO: Invalidate the login access_token
fn invalidate_login() {
    todo!();
}
