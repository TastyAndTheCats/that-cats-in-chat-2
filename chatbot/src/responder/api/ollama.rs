use api_consumers::ollama::talk_to_chatbot;
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::message::rest_of_chat_message;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("api::ollama::blueberry") {
        format!(
            "Blueberry says, to {{sender}}: {}",
            talk_to_chatbot("blueberry", rest_of_chat_message(msg, command).to_owned()).await
        )
    } else if response_fn.starts_with("api::ollama::friday") {
        format!(
            "Friday says, to {{sender}}: {}",
            talk_to_chatbot("friday", rest_of_chat_message(msg, command).to_owned()).await
        )
    } else if response_fn.starts_with("api::ollama::zoey") {
        format!(
            "Zoey says, to {{sender}}: {}",
            talk_to_chatbot("zoey", rest_of_chat_message(msg, command).to_owned()).await
        )
    } else {
        String::from("Unknown Function (epic_store)")
    }
}
