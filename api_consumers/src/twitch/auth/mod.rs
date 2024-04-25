//! API Routes for Logging Into Twitch

use reqwest::{Client, Error, Response};
use types::get;
use utils::serde_json::unwrap_reqwest;

pub mod token;

/// Generates a random string for nonce purposes
pub fn state() -> String {
    utils::rand::generate_password(46)
}

/// Easy place to understand where these credentials are coming from
pub fn credentials_url(state: &str, scope: &str, redirect_uri: &str) -> String {
    let base_url = "https://id.twitch.tv/oauth2/authorize";
    let app = get::app(None, None, None, None);
    let force_verify = "false";
    let response_type = "code";
    utils::url::construct_url(
        base_url,
        vec![
            ("client_id", &app.client_id),
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
    let app = get::app(None, None, None, None);
    let params = vec![
        ("client_id", app.client_id),
        ("client_secret", app.client_secret),
        ("code", code.to_owned()),
        ("grant_type", "authorization_code".to_owned()),
        ("redirect_uri", app.login_redirect_url),
    ];

    Client::new()
        .post("https://id.twitch.tv/oauth2/token")
        .body(utils::url::compose_post_body(&params))
        .send()
        .await
}

/// This validates the access_token AND gives you the user's login and id
pub async fn validate_access_token(access_token: &str) -> Result<Response, Error> {
    Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("OAuth {}", access_token))
        .send()
        .await
}

/// Checks if the login is valid
pub async fn get_userid_and_login_from_validated_access_token(
    access_token: &str,
) -> [Option<String>; 2] {
    let validation_json = unwrap_reqwest(validate_access_token(&access_token).await).await;
    if validation_json["status"] == 401 {
        return [None, None];
    }
    return [
        Some(validation_json["user_id"].as_str().unwrap().to_owned()),
        Some(validation_json["login"].as_str().unwrap().to_owned()),
    ];
}
