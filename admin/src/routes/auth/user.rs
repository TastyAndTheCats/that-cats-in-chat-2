//! Contains all of the Twitch-user-auth related routes

use super::validate_twitch_login;
use actix_web::{
    get,
    web::{redirect, Query},
    HttpResponse, Responder,
};
use api_consumers::twitch::auth;
use database::login;
use types::{
    auth::{TwitchLoginFailResponse, TwitchLoginSuccessResponse},
    get::app,
    twitch::App,
};

/// Hit this route to initiate a login request
#[get("/login")]
async fn twitch_login_initiate() -> impl Responder {
    let state = auth::state();
    let scope = "channel:bot moderator:manage:shoutouts";
    let app: App = app(None, None, None, None);
    login::user::initiate_login(&state, &scope, true, false).unwrap();
    redirect(
        "/login",
        auth::credentials_url(&state, &scope, &app.login_redirect_url),
    )
}

/// Twitch will redirect to here after the process complete, success or fail
#[get("/login_accepted/")] // The ending / is required for Twitch reasons
async fn twitch_login_accepted(query: Query<TwitchLoginSuccessResponse>) -> impl Responder {
    let [user_id, user_login] = validate_twitch_login(&query).await;

    let user_id = user_id.expect("Login was invalid").parse::<i32>().unwrap();
    login::user::save_initial_user_details(
        &query.state,
        &user_id,
        user_login.expect("Login was invalid").as_str(),
    )
    .unwrap();

    redirect("/login_accepted", format!("/dashboard/{}", user_id))
}

// This might overright the above route on a fail? I would like that, but I have doubts
#[get("/login_accepted/")] // The ending / is required for Twitch reasons
                           // TODO: test this
async fn twitch_login_rejected(query: Query<TwitchLoginFailResponse>) -> impl Responder {
    login::user::twitch_login_failed(&query.state, &query.error, &query.error_description).unwrap();
    HttpResponse::Ok().body(format!(
        "Login accepted. error:{} error_description:{} state:{}",
        query.error, query.error_description, query.state
    ))
}
