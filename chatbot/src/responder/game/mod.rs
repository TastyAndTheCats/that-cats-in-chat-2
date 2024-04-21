mod colony;

use database::models::responders::TwitchResponder;

pub async fn dispatch(responder: &TwitchResponder, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("games::colony") {
        return colony::dispatch(responder, command).await;
    } else {
        return "Unknown Function (colony)".to_owned();
    }
}
