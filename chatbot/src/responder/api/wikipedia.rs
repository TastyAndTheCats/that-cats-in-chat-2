use api_consumers::wikipedia::lookup;
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::message::rest_of_chat_message;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();

    if response_fn.starts_with("api::wikipedia::lookup") {
        cmd_wikipedia_lookup(msg, command).await
    } else {
        String::from("Unknown Function (wikipedia)")
    }
}
async fn cmd_wikipedia_lookup(msg: &PrivmsgMessage, command: &str) -> String {
    let title = rest_of_chat_message(msg, command).to_owned();
    tracing::info!("Wikipedia article lookup: '{}'", title);
    lookup(title).await
}
