//! Twitch API calls

use std::env;

use reqwest::{Client, Error, Response, StatusCode};

use types::get;
use utils::{twitch, url::compose_post_body};

pub mod auth; // Twitch Auth Routes
pub mod channel;
pub mod former_slash_actions;
pub mod users; // Twitch User Related Lookups // Twitch Channel Related Lookups

enum RequestVerb {
    GET,
    POST,
    PATCH,
}

/// GET:
pub async fn get(url: &str, user_id: Option<i32>) -> Result<Response, Error> {
    request(RequestVerb::GET, url, user_id, None).await
}

pub async fn post(
    url: &str,
    params: Option<Vec<(&str, String)>>,
    user_id: Option<i32>,
) -> Result<Response, Error> {
    request(RequestVerb::POST, url, user_id, params).await
}

pub async fn patch(
    url: &str,
    params: Option<Vec<(&str, String)>>,
    user_id: Option<i32>,
) -> Result<Response, Error> {
    request(RequestVerb::PATCH, url, user_id, params).await
}

/// If the response is a 401, refresh the tokens and try again
async fn request(
    verb: RequestVerb,
    url: &str,
    user_id: Option<i32>,
    params: Option<Vec<(&str, String)>>,
) -> Result<Response, Error> {
    let params = params.unwrap_or(vec![]);
    let result = match verb {
        RequestVerb::GET => get_req(url).await.unwrap(),
        RequestVerb::POST => post_req(url, params).await.unwrap(),
        RequestVerb::PATCH => patch_req(url, params).await.unwrap(),
    };
    // let result = post_req(url, params, user_id).await.unwrap();
    match result.status() {
        StatusCode::OK | StatusCode::NO_CONTENT => Ok(result),
        StatusCode::UNAUTHORIZED => {
            let user_id = match user_id {
                Some(user_id) => user_id,
                None => env::var("DEFAULT_BOT_ID")
                    .unwrap_or("0".to_owned())
                    .parse::<i32>()
                    .unwrap_or(0),
            };
            auth::token::refresh(user_id).await;
            get_req(url).await
        }
        _ => {
            tracing::warn!("Unhandled response code {}", result.status());
            Ok(result)
        }
    }
}

/// GET: If the response is invalid, refresh the tokens and try again
async fn get_req(url: &str) -> Result<Response, Error> {
    let (client_id, access_token) = twitch::client_and_access_token();
    Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", access_token.unwrap_or(String::new())),
        )
        .header("Client-Id", client_id)
        .send()
        .await
}

/// POST: If the response is invalid, refresh the tokens and try again
async fn post_req(url: &str, params: Vec<(&str, String)>) -> Result<Response, Error> {
    let (client_id, access_token) = twitch::client_and_access_token();
    Client::new()
        .post(url)
        .header(
            "Authorization",
            format!("Bearer {}", access_token.unwrap_or(String::new())),
        )
        .header("Client-Id", client_id)
        .body(compose_post_body(&params))
        .send()
        .await
}

/// PATCH: If the response is invalid, refresh the tokens and try again
async fn patch_req(url: &str, params: Vec<(&str, String)>) -> Result<Response, Error> {
    let (client_id, access_token) = twitch::client_and_access_token();
    Client::new()
        .patch(url)
        .header(
            "Authorization",
            format!("Bearer {}", access_token.unwrap_or(String::new())),
        )
        .header("Client-Id", client_id)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(compose_post_body(&params))
        .send()
        .await
}
