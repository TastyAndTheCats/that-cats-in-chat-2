use actix_web::{
    get,
    web::{redirect, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};
use api_consumers::twitch::{self, auth};
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

/// Shared portion of handling the user and bot logins
async fn validate_twitch_login(
    query: &Query<auth::TwitchLoginSuccessResponse>,
) -> [Option<String>; 2] {
    login::twitch_login_successful(&query.state, &query.scope, &query.code);
    is_twitch_login_valid(&get_access_token_from_twitch(&query).await).await
}

/// Do Twitch secret handshake
async fn get_access_token_from_twitch(query: &Query<auth::TwitchLoginSuccessResponse>) -> String {
    let handshake_json: serde_json::Value = serde_json::from_str(
        &auth::complete_handshake(&query.code)
            .await
            .expect("Secret handshake with Twitch failed")
            .text()
            .await
            .unwrap(),
    )
    .unwrap();
    let access_token = handshake_json["access_token"]
        .as_str()
        .expect("No access_token provided in secret handshake response");

    login::save_new_access_token(
        &query.state,
        &handshake_json["refresh_token"].as_str().unwrap(),
        &handshake_json["expires_in"].to_string(),
        &access_token,
        &handshake_json["token_type"].as_str().unwrap(),
    );

    access_token.to_owned()
}

/// Checks if the login is valid
async fn is_twitch_login_valid(access_token: &str) -> [Option<String>; 2] {
    let validation_json: serde_json::Value = serde_json::from_str(
        &twitch::auth::validate_access_token(&access_token)
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    )
    .unwrap();
    if validation_json["status"] == 401 {
        invalidate_login();
        return [None, None];
    }
    return [
        Some(validation_json["user_id"].as_str().unwrap().to_owned()),
        Some(validation_json["login"].as_str().unwrap().to_owned()),
    ];
}

/// Invalidate the login access_token
fn invalidate_login() {}

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

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(twitch_login_initiate)
        .service(twitch_login_accepted)
        .service(twitch_login_rejected)
        .service(twitch_bot_login_initiate)
        .service(twitch_bot_login_accepted)
        .service(twitch_bot_login_rejected);
}
