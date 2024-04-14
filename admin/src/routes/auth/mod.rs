//! Contains all of the Twitch-auth related routes in the TCIC Admin application

use actix_web::web::{Query, ServiceConfig};
use api_consumers::twitch::{self, auth};
use database::login;

mod bot;
mod user;

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

/// TODO: Invalidate the login access_token
fn invalidate_login() {
    todo!();
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(user::twitch_login_initiate)
        .service(user::twitch_login_accepted)
        .service(user::twitch_login_rejected)
        .service(bot::twitch_bot_login_initiate)
        .service(bot::twitch_bot_login_accepted)
        .service(bot::twitch_bot_login_rejected);
}
