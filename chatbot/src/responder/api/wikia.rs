use api_consumers::wikia;
use chrono::{DateTime, NaiveDateTime, Utc};
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::serde_json::unwrap_reqwest;

pub async fn dispatch(
    responder: &TwitchResponder,
    _msg: &PrivmsgMessage,
    _command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("api::wikia::lookup") {
        cmd_wikia_lookup().await
    } else {
        String::from("Unknown Function (wikia)")
    }
}

async fn cmd_wikia_lookup() -> String {
    let lookup = unwrap_reqwest(wikia::lookup("dwarffortress", "limestone").await).await;
    tracing::debug!("{:?}", lookup);
    String::new()
}
