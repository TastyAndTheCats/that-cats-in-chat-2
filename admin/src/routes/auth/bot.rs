//! Contains all of the Twitch-chatbot-auth related routes

use actix_web::{
    get,
    web::{redirect, Path, Query},
    HttpResponse, Responder,
};
use tracing;

use api_consumers::twitch::auth;
use database::login;

use super::validate_twitch_login;
use types::{
    auth::TwitchLoginFailResponse, auth::TwitchLoginSuccessResponse, get::app, twitch::App,
};

/// Hit this route to initiate a bot login request
#[get("/bot_login/{owner_id}")]
async fn twitch_bot_login_initiate(path: Path<i32>) -> impl Responder {
    let state = auth::state();
    let scope = "chat:read chat:edit";
    let owner_id = path.into_inner();
    let app: App = app(None, None, None, None);

    login::user::initiate_login(&state, &scope, false, true).unwrap();
    login::bot::add_bot_owner(&state, &owner_id).unwrap();

    redirect(
        "/bot_login",
        auth::credentials_url(&state, &scope, &app.bot_login_redirect_url),
    )
}

/// Twitch will redirect here after the bot login process is complete
#[get("/bot_login_accepted/")] // The ending / is required for Twitch reasons
async fn twitch_bot_login_accepted(query: Query<TwitchLoginSuccessResponse>) -> impl Responder {
    let [bot_id, bot_login] = validate_twitch_login(&query).await;

    let bot_id = utils::parse_id(bot_id.expect("Login was invalid"));
    let bot_owner = database::channel::bot_owner_from_state(&query.state)
        .expect("State was invalid, probably a db error");
    let bot_login = bot_login.expect("Login was invalid");

    let results = login::bot::save_initial_bot_details(
        &query.state,
        &bot_id,
        &bot_login,
        &bot_owner.channel_id,
    )
    .expect("Initial bot details couldn't be saved");
    if results > 1 {
        tracing::warn!("login::bot::save_initial_bot_details altered more than one row");
    }

    HttpResponse::Ok().body(format!(
        "You have succesfully registered {} as a chatbot powered by TheCatsInChat (in channel #{})!\n You can close this tab/window.",
        bot_login, bot_owner.channel_id
    ))
}

// This might overright the above route on a fail? I would like that, but I have doubts
#[get("/bot_login_accepted/")] // The ending / is required for Twitch reasons
                               // TODO: test this
async fn twitch_bot_login_rejected(query: Query<TwitchLoginFailResponse>) -> impl Responder {
    login::user::twitch_login_failed(&query.state, &query.error, &query.error_description).unwrap();
    HttpResponse::Ok().body(format!(
        "Login accepted. error:{} error_description:{} state:{}",
        query.error, query.error_description, query.state
    ))
}
