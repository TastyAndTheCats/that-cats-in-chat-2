use twitch_irc::message::PrivmsgMessage;

use api_consumers::twitch::{channel::lookup_channel_from_id, users::lookup_user_from_login};
use database::models::responders::TwitchResponder;
use utils::message::single_word_after_command;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::niceties::shoutout") {
        return cmd_shoutout(msg, command).await;
    } else {
        return "Unknown Function".to_owned();
    }
}

async fn cmd_shoutout(msg: &PrivmsgMessage, command: &str) -> String {
    let user_json: serde_json::Value = serde_json::from_str(
        &lookup_user_from_login(&single_word_after_command(msg, command))
            .await
            .text()
            .await
            .unwrap(),
    )
    .unwrap();

    let user = &user_json["data"][0];
    let so_id = user["id"].as_str().unwrap();

    let channel_json: serde_json::Value =
        serde_json::from_str(&lookup_channel_from_id(so_id).await.text().await.unwrap()).unwrap();
    let channel = &channel_json["data"][0];

    let game_name = channel["game_name"].as_str().unwrap();
    let so_name = user["display_name"].as_str().unwrap();
    let so_description = user["description"].as_str().unwrap();

    format!(
        "Go check out {} at twitch.tv/{} and make a new friend?! {} was last seen streaming {}! ⟹⟹⟹ {}",
        so_name,
        so_name,
        so_name,
        game_name,
        so_description,
    )
}
