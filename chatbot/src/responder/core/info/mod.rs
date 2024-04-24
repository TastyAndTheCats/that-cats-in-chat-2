use chrono::Local;
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use utils::rand::random_from_vec;

pub async fn dispatch(
    responder: &TwitchResponder,
    _msg: &PrivmsgMessage,
    _command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "core::info" {
        return cmd_info();
    } else if response_fn.starts_with("core::info::time") {
        return cmd_time();
    } else {
        return "Unknown Function (info)".to_owned();
    }
}

fn cmd_info() -> String {
    let possible_responses = Vec::from([
        "are what happens when @TastyAndTheCats lets his cats go unsupervised",
        "are a little bit of an experiment.",
        "can't be stopped. Get on their good side by subscribing or giving bits.",
        "are taking the first steps towards complete global domination.",
    ]);

    random_from_vec(&possible_responses).unwrap().to_string()
}

fn cmd_time() -> String {
    format!(
        "It's currently {} for {{channel_name}}",
        Local::now().format("%H:%M")
    )
}
