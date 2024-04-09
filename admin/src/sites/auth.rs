use actix_web::web::{redirect, Query, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use database::login;
use reqwest::{Client, Error, Response};
use serde::Deserialize;
use std::env;

fn state() -> String {
    utils::rand::generate_password(46)
}

/// Easy place to understand where these credentials are coming from
fn credentials_url(state: &str) -> String {
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

#[derive(Deserialize)]
struct TwitchLoginSuccessResponse {
    code: String,
    scope: String,
    state: String,
}

#[derive(Deserialize)]
struct TwitchLoginFailResponse {
    error: String,
    error_description: String,
    state: String,
}

async fn twitch_login_complete_handshake(code: &str) -> Result<Response, Error> {
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

#[get("/login")]
async fn twitch_login_initiate() -> impl Responder {
    let state = state();
    let scope = "";
    login::initiate_login(&state, &scope, true, false);
    redirect("/login", credentials_url(&state))
}

#[get("/login_accepted/")] // The ending / is required for Twitch reasons
async fn twitch_login_accepted(query: Query<TwitchLoginSuccessResponse>) -> impl Responder {
    login::twitch_login_successful(&query.state, &query.scope, &query.code);
    let handshake_response = twitch_login_complete_handshake(&query.code)
        .await
        .expect("Secret handshake with Twitch failed");
    let handshake_resp_json: serde_json::Value = serde_json::from_str(&handshake_response.text().await.unwrap()).unwrap();
    login::save_new_access_token();

    HttpResponse::Ok().body(format!(
        "Login accepted. code:{} scope:{} state:{}, handshake_resp: {:?}",
        query.code, query.scope, query.state, handshake_resp_json
    ))
}

#[get("/login_accepted/")] // The ending / is required for Twitch reasons
async fn twitch_login_rejected(query: Query<TwitchLoginFailResponse>) -> impl Responder {
    login::twitch_login_failed(&query.state, &query.error, &query.error_description);
    HttpResponse::Ok().body(format!(
        "Login accepted. error:{} error_description:{} state:{}",
        query.error, query.error_description, query.state
    ))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(twitch_login_initiate)
        .service(twitch_login_accepted)
        .service(twitch_login_rejected);
    // .service(hello)
    // .service(echo);
}
