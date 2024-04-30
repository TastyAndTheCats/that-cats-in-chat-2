use database::{
    models::responders::TwitchResponder, responder::get_user_responders_with_display_names,
};
use twitch_irc::message::PrivmsgMessage;
use utils::rand::shuffled_vec;

mod emoji;
mod facts;
mod info;
mod maths;
mod niceties;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "core::commands" {
        cmd_commands_list(msg)
    } else if response_fn.starts_with("core::emoji") {
        emoji::dispatch(responder, msg, command).await
    } else if response_fn.starts_with("core::facts") {
        facts::dispatch(responder, msg, command).await
    } else if response_fn.starts_with("core::info") {
        info::dispatch(responder, msg, command).await
    } else if response_fn.starts_with("core::maths") {
        maths::dispatch(responder, msg, command).await
    } else if response_fn.starts_with("core::niceties") {
        niceties::dispatch(responder, msg, command).await
    } else {
        "Unknown Function {core)".to_owned()
    }
}

fn cmd_commands_list(msg: &PrivmsgMessage) -> String {
    let command_names =
        get_user_responders_with_display_names(msg.channel_id.parse::<i32>().unwrap_or(0))
            .unwrap_or(vec![]);
    let mut command_names: Vec<String> = command_names
        .into_iter()
        .filter(|cmd| cmd.is_some())
        .map(|cmd| cmd.unwrap())
        .collect();
    command_names = shuffled_vec(command_names);

    command_names.join(" ")
}
