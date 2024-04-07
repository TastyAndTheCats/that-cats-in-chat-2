use actix_web::{get, web, HttpResponse, Responder};
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

#[get("/login_accepted/")]
async fn twitch_login_accepted() -> impl Responder {
    HttpResponse::Ok().body("Login accepted.")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let state = state();

    cfg.service(web::redirect("/login", credentials_url(&state)))
        .service(twitch_login_accepted);
    // .service(hello)
    // .service(echo);
}
