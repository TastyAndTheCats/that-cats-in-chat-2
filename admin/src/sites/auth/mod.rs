use actix_web::web::{redirect, Query, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use database::login;

mod twitch;

#[get("/login")]
async fn twitch_login_initiate() -> impl Responder {
    let state = twitch::state();
    let scope = "";
    login::initiate_login(&state, &scope, true, false);
    redirect("/login", twitch::credentials_url(&state))
}

#[get("/login_accepted/")] // The ending / is required for Twitch reasons
async fn twitch_login_accepted(query: Query<twitch::TwitchLoginSuccessResponse>) -> impl Responder {
    login::twitch_login_successful(&query.state, &query.scope, &query.code);

    let handshake_response = twitch::complete_handshake(&query.code)
        .await
        .expect("Secret handshake with Twitch failed");

    let handshake_resp_json: serde_json::Value =
        serde_json::from_str(&handshake_response.text().await.unwrap()).unwrap();

    login::save_new_access_token(
        &query.state,
        &handshake_resp_json["refresh_token"].to_string(),
        &handshake_resp_json["token_expiry"].to_string(),
        &handshake_resp_json["access_token"].to_string(),
        &handshake_resp_json["token_type"].to_string(),
    );

    

    HttpResponse::Ok().body(format!(
        "Login accepted. code:{} scope:{} state:{}, handshake_resp: {:?}",
        query.code, query.scope, query.state, handshake_resp_json
    ))
}

#[get("/login_accepted/")] // The ending / is required for Twitch reasons
                           // TODO: test this
async fn twitch_login_rejected(query: Query<twitch::TwitchLoginFailResponse>) -> impl Responder {
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
