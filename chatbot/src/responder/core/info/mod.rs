use chrono::Local;
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;

pub async fn dispatch(
    responder: &TwitchResponder,
    _msg: &PrivmsgMessage,
    _command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::info::time") {
        return cmd_time();
    } else {
        return "Unknown Function (info)".to_owned();
    }
}

fn cmd_time() -> String {
    format!(
        "It's currently {} for {{channel_name}}",
        Local::now().format("%H:%M")
    )
}
