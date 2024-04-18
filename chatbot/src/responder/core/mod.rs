mod niceties;

use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;

use crate::types::TwitchClientType;

pub async fn dispatch(
    client: &TwitchClientType,
    responder: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::niceties") {
        return niceties::dispatch(client, responder, msg, command).await;
    } else {
        return "Unknown Function {core)".to_owned();
    }
}
