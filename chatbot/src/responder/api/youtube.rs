use regex::Regex;

use api_consumers::youtube;
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::{message::rest_of_chat_message, serde_json::unwrap_reqwest};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, _command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();

    if response_fn.starts_with("api::youtube::get_video_info") {
        cmd_youtube_lookup(msg).await
    } else {
        String::from("Unknown Function (youtube)")
    }
}
async fn cmd_youtube_lookup(msg: &PrivmsgMessage) -> String {
    let re = Regex::new(r#"(?i)\b((?:https?://|www\d{0,3}[.]|[a-z0-9.\-]+[.][a-z]{2,4}/)(?:[^\s()<>]+|\(([^\s()<>]+|(\([^\s()<>]+\)))*\))+(?:\(([^\s()<>]+|(\([^\s()<>]+\)))*\)|[^\s`!()\[\]{};:'\".,<>?«»“”‘’]))"#).unwrap();
    let matches: Vec<&str> = re
        .find_iter(&msg.message_text)
        .map(|m| m.as_str())
        .collect();
    let mut titles = String::new();
    for m in matches {
        titles = format!(
            "{}{:#?} ",
            titles,
            unwrap_reqwest(youtube::get_video_snippet(m).await)
                .await
                .as_object()
                .unwrap()["items"][0]["snippet"]["title"]
                .as_str()
                .unwrap_or("")
        );
    }
    if titles.len() > 3 {
        format!("{{sender}} put a YouTube video titled {} in chat", titles)
    } else {
        String::new()
    }
}
