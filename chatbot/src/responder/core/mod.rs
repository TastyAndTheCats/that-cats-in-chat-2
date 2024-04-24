use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::rand::random_from_vec;

mod facts;
mod info;
mod niceties;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "core::commands" {
        cmd_commands_list()
    } else if response_fn.starts_with("core::facts") {
        facts::dispatch(responder, msg, command).await
    } else if response_fn.starts_with("core::info") {
        info::dispatch(responder, msg, command).await
    } else if response_fn.starts_with("core::niceties") {
        niceties::dispatch(responder, msg, command).await
    } else {
        "Unknown Function {core)".to_owned()
    }
}

fn cmd_commands_list() -> String {
    let possible_responses = Vec::from(["This is under construction"]);

    random_from_vec(&possible_responses).unwrap().to_string()
}
