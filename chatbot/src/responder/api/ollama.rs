use api_consumers::ollama::{LlamaBot, Talk};
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::message::rest_of_chat_message;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("api::ollama::blueberry") {
        talk_to("Blueberry", "blueberry", msg, command).await
    } else if response_fn.starts_with("api::ollama::friday") {
        talk_to("Friday", "friday", msg, command).await
    } else if response_fn.starts_with("api::ollama::zoey") {
        talk_to("Zoey", "zoey", msg, command).await
    } else {
        String::from("Unknown Function (epic_store)")
    }
}

async fn talk_to(cat: &str, model: &str, msg: &PrivmsgMessage, command: &str) -> String {
    let llama_bot = LlamaBot { model: model };
    format!(
        "{} says, to {{sender}}: {}",
        cat,
        llama_bot
            .talk(rest_of_chat_message(msg, command).to_owned())
            .await
    )
}
