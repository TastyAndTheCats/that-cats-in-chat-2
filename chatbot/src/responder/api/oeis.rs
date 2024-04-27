use twitch_irc::message::PrivmsgMessage;

use api_consumers::oeis::{get_sequence, random_sequence};
use database::models::responders::TwitchResponder;
use utils::{message::first_word_after_command_as_number, serde_json::unwrap_reqwest};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("api::oeis::lookup") {
        return cmd_oeis(msg, command).await;
    } else {
        return "Unknown Function (oeis)".to_owned();
    }
}

async fn cmd_oeis(msg: &PrivmsgMessage, command: &str) -> String {
    let sequence_id = match first_word_after_command_as_number(msg, command) {
        Ok(num) => num,
        Err(_) => -1,
    };

    let sequence_data = if sequence_id < 1 {
        random_sequence().await
    } else {
        get_sequence(sequence_id).await
    };
    let json_data = unwrap_reqwest(sequence_data).await;
    let json_data = &json_data["results"][0].as_object().unwrap();

    let sequence_id = json_data["number"].as_i64().unwrap_or(-1);
    let sequence_name = json_data["name"].as_str().unwrap_or("Untitled");
    let sequence_example = json_data["data"]
        .as_str()
        .unwrap()
        .split(",")
        .collect::<Vec<&str>>()
        .join(", ");
    format!(
        "OEIS sequence for {{sender}}:  A{}: {} => {}",
        sequence_id, sequence_name, sequence_example
    )
}
