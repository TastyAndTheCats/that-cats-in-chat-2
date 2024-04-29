//! Contains all of the Twitch-auth related routes in the TCIC Admin application

use actix_web::web::{Query, ServiceConfig};
use api_consumers::twitch::auth;
use database::login;
use types::auth::TwitchLoginSuccessResponse;
use utils::serde_json::unwrap_reqwest;

mod bot;
mod user;

/// Shared portion of handling the user and bot logins
async fn validate_twitch_login(query: &Query<TwitchLoginSuccessResponse>) -> [Option<String>; 2] {
    login::user::twitch_login_successful(&query.state, &query.scope, &query.code)
        .expect("Unable to validate twitch login");
    auth::get_userid_and_login_from_validated_access_token(
        &get_access_token_from_twitch(&query).await,
    )
    .await
}

/// Do Twitch's secret handshake
async fn get_access_token_from_twitch(query: &Query<TwitchLoginSuccessResponse>) -> String {
    let handshake_json = unwrap_reqwest(auth::complete_handshake(&query.code).await).await;
    let access_token = handshake_json["access_token"]
        .as_str()
        .expect("No access_token provided in secret handshake response");

    login::user::save_new_access_token(
        &query.state,
        &handshake_json["refresh_token"].as_str().unwrap(),
        &handshake_json["expires_in"].to_string(),
        &access_token,
        &handshake_json["token_type"].as_str().unwrap(),
    )
    .expect("Unable to save new login information");

    access_token.to_owned()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(user::twitch_login_initiate)
        .service(user::twitch_login_accepted)
        .service(user::twitch_login_rejected)
        .service(bot::twitch_bot_login_initiate)
        .service(bot::twitch_bot_login_accepted)
        .service(bot::twitch_bot_login_rejected);
}
