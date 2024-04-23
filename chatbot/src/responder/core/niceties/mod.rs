use twitch_irc::message::PrivmsgMessage;

use api_consumers::twitch::{
    channel::lookup_channel_from_id, former_slash_actions::shoutout, users::lookup_user_from_login,
};
use database::models::responders::TwitchResponder;
use utils::{message::single_word_after_command, serde_json::unwrap_reqwest};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::niceties::shoutout") {
        return cmd_shoutout(msg, command).await;
    } else {
        return "Unknown Function (niceties)".to_owned();
    }
}

async fn cmd_shoutout(msg: &PrivmsgMessage, command: &str) -> String {
    let user_json: serde_json::Value =
        unwrap_reqwest(lookup_user_from_login(&single_word_after_command(msg, command)).await)
            .await;

    let user = &user_json["data"][0];
    let so_id = user["id"].as_str().unwrap();

    let channel_json = unwrap_reqwest(lookup_channel_from_id(so_id).await).await;
    let channel = &channel_json["data"][0];

    let game_name = channel["game_name"].as_str().unwrap();
    let so_name = user["display_name"].as_str().unwrap();
    let so_description = user["description"].as_str().unwrap();

    let mut game_message = format!("{} hasn't streamed (yet), but they might!", so_name);
    if game_name != "" {
        game_message = format!("{} was last seen streaming {}!", so_name, game_name);
    }

    match shoutout(&msg.channel_id, so_name).await {
        Ok(resp) => {
            println!("resp status: {}", resp.status());
            println!("resp: {:?}", resp);
            format!("{} ⟹⟹⟹ {}", game_message, so_description,)
        }
        Err(_) => {
            format!(
                "Go check out {} at twitch.tv/{} and make a new friend?! {} ⟹⟹⟹ {}",
                so_name, so_name, game_message, so_description,
            )
        }
    }
}
