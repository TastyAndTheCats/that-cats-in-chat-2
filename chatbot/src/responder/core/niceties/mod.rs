use twitch_irc::message::PrivmsgMessage;

use api_consumers::twitch::users::lookup_user_from_login;
use database::models::responders::TwitchResponder;
use utils::single_word_after_command;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::niceties::shoutout") {
        return cmd_shoutout(msg, command).await;
    } else {
        return "Unknown Function".to_owned();
    }
}

async fn cmd_shoutout(msg: &PrivmsgMessage, command: &str) -> String {
    let shoutout_for = single_word_after_command(msg, command);
    let user_json: serde_json::Value = serde_json::from_str(
        &lookup_user_from_login(&shoutout_for)
            .await
            .text()
            .await
            .unwrap(),
    )
    .unwrap();
    let user = &user_json["data"][0];
    println!(
        "shoutout for: {} results in a lookup response {:?}",
        &shoutout_for, &user
    );
    let message = "GO FOLLOW";
    return format!(
        "{} twitch.tv/{} {}",
        message,
        &user["display_name"].as_str().unwrap(),
        &user["description"].as_str().unwrap(),
    );
}
