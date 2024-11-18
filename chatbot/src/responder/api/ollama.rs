use api_consumers::ollama::{LlamaBot,Talk};
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::{message::rest_of_chat_message, rand::random_from_vec};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    let cat_models = vec![
        ("Blueberry", "blueberry"),
        ("Friday", "friday"),
        ("Zoey", "zoey"),
    ];
    let mut chosen_cat = cat_models[0];
    if response_fn.starts_with("api::ollama::blueberry") {
        chosen_cat = cat_models[0];
    } else if response_fn.starts_with("api::ollama::friday") {
        chosen_cat = cat_models[1];
    } else if response_fn.starts_with("api::ollama::zoey") {
        chosen_cat = cat_models[2];
    } else if response_fn.starts_with("api::ollama::chat") {
        chosen_cat = *random_from_vec(&cat_models).unwrap();
    }
    talk_to(chosen_cat.0, chosen_cat.1, msg, command).await
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
