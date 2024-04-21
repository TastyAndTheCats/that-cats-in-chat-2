use twitch_irc::message::PrivmsgMessage;

use api_consumers::twitch::{channel::lookup_channel_from_id, users::lookup_user_from_login};
use database::models::responders::TwitchResponder;
use utils::message::single_word_after_command;

use crate::local_types::TwitchClient;

pub async fn dispatch(
    client: &TwitchClient,
    responder: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::niceties::shoutout") {
        return cmd_shoutout(client, msg, command).await;
    } else {
        return "Unknown Function (niceties)".to_owned();
    }
}

async fn cmd_shoutout(client: &TwitchClient, msg: &PrivmsgMessage, command: &str) -> String {
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

    let channel_json: serde_json::Value = serde_json::from_str(
        &lookup_channel_from_id(so_id)
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    )
    .unwrap();
    let channel = &channel_json["data"][0];

    let game_name = channel["game_name"].as_str().unwrap();
    let so_name = user["display_name"].as_str().unwrap();
    let so_description = user["description"].as_str().unwrap();

    let mut game_message = format!("{} hasn't streamed (yet), but they might!", so_name);
    if game_name != "" {
        game_message = format!("{} was last seen streaming {}!", so_name, game_name);
    }

    do_builtin_twitch_shoutout(client, msg, so_name).await;
    format!(
        "Go check out {} at twitch.tv/{} and make a new friend?! {} ⟹⟹⟹ {}",
        so_name, so_name, game_message, so_description,
    )
}

async fn do_builtin_twitch_shoutout(client: &TwitchClient, msg: &PrivmsgMessage, command: &str) {
    // NOTE: Doesn't seem to work - because this is deprecated and we have to use the API now
    // client
    //     .privmsg(
    //         msg.channel_login.to_owned(),
    //         format!("/shoutout {}", command),
    //     )
    //     .await
    //     .unwrap();
}
