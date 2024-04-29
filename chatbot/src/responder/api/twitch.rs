use api_consumers::twitch::channel::{set_game, set_title};
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::{message::rest_of_chat_message, serde_json::unwrap_reqwest};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();

    if response_fn.starts_with("api::twitch::set_game") {
        cmd_set_game(msg, command).await
    } else if response_fn.starts_with("api::twitch::set_title") {
        cmd_set_title(msg, command).await
    } else {
        String::from("Unknown Function (twitch)")
    }
}
async fn cmd_set_game(msg: &PrivmsgMessage, command: &str) -> String {
    let title = rest_of_chat_message(msg, command).to_owned();
    tracing::info!("set_game: '{}'", title);
    let resp = unwrap_reqwest(set_game(&msg.channel_id, &title).await).await;

    println!("{:?}", resp);

    format!("Attempted to set game to {}", title)
}

async fn cmd_set_title(msg: &PrivmsgMessage, command: &str) -> String {
    let title = rest_of_chat_message(msg, command).to_owned();
    tracing::info!("set_title: '{}'", title);
    let resp = unwrap_reqwest(set_title(&msg.channel_id, &title).await).await;

    println!("{:?}", resp);

    format!("Attempted to set title to {}", title)
}
