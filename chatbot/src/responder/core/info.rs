use api_consumers::time::time_at_location;
use chrono::{DateTime, Local, Month};
use serde_json::Value;
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use utils::{
    message::rest_of_chat_message, num::ordinal, rand::random_from_vec, serde_json::unwrap_reqwest,
};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "core::info" {
        cmd_info()
    } else if response_fn == "core::info::time" {
        cmd_time(msg, command).await
    } else {
        "Unknown Function (info)".to_owned()
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

async fn cmd_time(msg: &PrivmsgMessage, command: &str) -> String {
    let mut rest_of_message = rest_of_chat_message(msg, command);
    if rest_of_message.len() <= 1 {
        rest_of_message = String::from("Toronto");
    }

    let time_at_req = unwrap_reqwest(time_at_location(&rest_of_message).await).await;
    convert_time_at_location_request_to_string(time_at_req)
}

fn convert_time_at_location_request_to_string(resp: Value) -> String {
    let hour = resp["hour"].as_i64().unwrap_or(45);
    format!(
        "{} {} of {} {} at {} {} in {}",
        resp["dayOfWeek"].as_str().unwrap_or("Someday"),
        ordinal(resp["day"].as_i64().unwrap_or(32).try_into().unwrap_or(33)),
        Month::try_from(resp["month"].as_i64().unwrap_or(1).try_into().unwrap_or(1) - 1)
            .unwrap()
            .name(),
        resp["year"].as_i64().unwrap_or(5191),
        resp["time"].as_str().unwrap_or("25:69"),
        if hour > 12 {
            format!("({}:{} PM)", hour - 12, resp["minute"])
        } else {
            String::new()
        },
        format!(
            "{}{}",
            resp["timeZone"].as_str().unwrap_or("Antarctica/Troll"),
            if resp["dstActive"].as_bool().unwrap_or(false) {
                String::from(" DST")
            } else {
                String::new()
            }
        )
    )
}
