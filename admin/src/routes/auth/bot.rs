//! Contains all of the Twitch-chatbot-auth related routes

use actix_web::{
    get,
    web::{redirect, Path, Query},
    HttpResponse, Responder,
};
use api_consumers::twitch::auth;
use database::login;
use std::env;

use super::validate_twitch_login;

/// Hit this route to initiate a bot login request
#[get("/bot_login/{owner_id}")]
async fn twitch_bot_login_initiate(path: Path<i32>) -> impl Responder {
    let state = auth::state();
    let scope = "chat:read chat:edit";
    let owner_id = path.into_inner();

    login::initiate_login(&state, &scope, false, true);
    login::add_bot_owner(&state, &owner_id);

    redirect(
        "/bot_login",
        auth::credentials_url(
            &state,
            &scope,
            &env::var("TWITCH_BOT_REDIRECT_URL")
                .expect("Missing TWITCH_BOT_REDIRECT_URL environment variable."),
        ),
    )
}

/// Twitch will redirect here after the bot login process is complete
#[get("/bot_login_accepted/")] // The ending / is required for Twitch reasons
async fn twitch_bot_login_accepted(
    query: Query<auth::TwitchLoginSuccessResponse>,
) -> impl Responder {
    let [bot_id, bot_login] = validate_twitch_login(&query).await;

    let bot_id = utils::parse_id(bot_id.expect("Login was invalid"));
    let bot_owner_id = database::channel::bot_owner_from_state(&query.state).await;
    let bot_login = bot_login.expect("Login was invalid");

    database::login::save_initial_bot_details(&query.state, &bot_id, &bot_login, &bot_owner_id);

    HttpResponse::Ok().body(format!(
        "You have succesfully registered {} as a chatbot powered by TheCatsInChat (in channel #{})!\n You can close this tab/window.",
        bot_login, bot_owner_id
    ))
}

// This might overright the above route on a fail? I would like that, but I have doubts
#[get("/bot_login_accepted/")] // The ending / is required for Twitch reasons
                               // TODO: test this
async fn twitch_bot_login_rejected(query: Query<auth::TwitchLoginFailResponse>) -> impl Responder {
    login::twitch_login_failed(&query.state, &query.error, &query.error_description);
    HttpResponse::Ok().body(format!(
        "Login accepted. error:{} error_description:{} state:{}",
        query.error, query.error_description, query.state
    ))
}
