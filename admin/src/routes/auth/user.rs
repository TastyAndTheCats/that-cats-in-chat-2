//! Contains all of the Twitch-user-auth related routes

use super::validate_twitch_login;
use actix_web::{
    get,
    web::{redirect, Query},
    HttpResponse, Responder,
};
use api_consumers::twitch::auth;
use database::login;
use std::env;

/// Hit this route to initiate a login request
#[get("/login")]
async fn twitch_login_initiate() -> impl Responder {
    let state = auth::state();
    let scope = "channel:bot";
    login::initiate_login(&state, &scope, true, false);
    redirect(
        "/login",
        auth::credentials_url(
            &state,
            &scope,
            &env::var("TWITCH_REDIRECT_URL")
                .expect("Missing TWITCH_REDIRECT_URL environment variable."),
        ),
    )
}

/// Twitch will redirect to here after the process complete, success or fail
#[get("/login_accepted/")] // The ending / is required for Twitch reasons
async fn twitch_login_accepted(query: Query<auth::TwitchLoginSuccessResponse>) -> impl Responder {
    let [user_id, user_login] = validate_twitch_login(&query).await;

    let user_id = user_id.expect("Login was invalid").parse::<i32>().unwrap();
    database::login::save_initial_user_details(
        &query.state,
        &user_id,
        user_login.expect("Login was invalid").as_str(),
    );

    redirect("/login_accepted", format!("/dashboard/{}", user_id))
}

// This might overright the above route on a fail? I would like that, but I have doubts
#[get("/login_accepted/")] // The ending / is required for Twitch reasons
                           // TODO: test this
async fn twitch_login_rejected(query: Query<auth::TwitchLoginFailResponse>) -> impl Responder {
    login::twitch_login_failed(&query.state, &query.error, &query.error_description);
    HttpResponse::Ok().body(format!(
        "Login accepted. error:{} error_description:{} state:{}",
        query.error, query.error_description, query.state
    ))
}
