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
}

/// GET: If the response is invalid, refresh the tokens and try again
pub async fn get(url: &str, user_id: Option<i32>) -> Result<Response, Error> {
    request(RequestVerb::GET, url, user_id, None).await
}

pub async fn post(
    url: &str,
    params: Vec<(&str, String)>,
    user_id: Option<i32>,
) -> Result<Response, Error> {
    request(RequestVerb::POST, url, user_id, Some(params)).await
}

async fn request(
    verb: RequestVerb,
    url: &str,
    user_id: Option<i32>,
    params: Option<Vec<(&str, String)>>,
) -> Result<Response, Error> {
    let user_id = match user_id {
        Some(user_id) => user_id,
        None => env::var("DEFAULT_BOT_ID")
            .unwrap_or("0".to_owned())
            .parse::<i32>()
            .unwrap_or(0),
    };
    let result = match verb {
        RequestVerb::GET => get_req(url, user_id).await.unwrap(),
        RequestVerb::POST => post_req(url, params.unwrap_or(vec![]), user_id)
            .await
            .unwrap(),
    };
    // let result = post_req(url, params, user_id).await.unwrap();
    match result.status() {
        StatusCode::OK => Ok(result),
        StatusCode::UNAUTHORIZED => {
            auth::token::refresh(user_id).await;
            get_req(url, user_id).await
        }
        _ => {
            tracing::warn!("Unhandled response code {}", result.status());
            Ok(result)
        }
    }
}

async fn get_req(url: &str, user_id: i32) -> Result<Response, Error> {
    let (client_id, access_token) = twitch::client_and_access_token(Some(user_id));
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
async fn post_req(url: &str, params: Vec<(&str, String)>, user_id: i32) -> Result<Response, Error> {
    let (client_id, access_token) = twitch::client_and_access_token(Some(user_id));
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
