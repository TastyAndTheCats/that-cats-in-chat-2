mod epic_store;
mod openweathermap;

use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("api::epic_store") {
        return epic_store::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::openweathermap") {
        return openweathermap::dispatch(responder, msg, command).await;
    } else {
        return "Unknown Function (api)".to_owned();
    }
}
