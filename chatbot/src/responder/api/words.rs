use api_consumers::words::{define, explore};
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::{message::rest_of_chat_message, serde_json::unwrap_reqwest};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();

    if response_fn.starts_with("api::words::dictionary") {
        cmd_dictionary_lookup(msg, command).await
    } else if response_fn.starts_with("api::words::thesaurus") {
        cmd_thesaurus_lookup(msg, command).await
    } else {
        String::from("Unknown Function (words)")
    }
}
async fn cmd_dictionary_lookup(msg: &PrivmsgMessage, command: &str) -> String {
    let word = rest_of_chat_message(msg, command).to_owned();
    tracing::info!("Dictionary lookup: '{}'", word);
    let resp = unwrap_reqwest(define(&word).await).await;
    let resp_obj = resp.as_object().unwrap();
    tracing::debug!("dict: {:#?}", resp_obj);

    let mut count = 1;
    let mut resp = format!(
        "{} [{}]",
        resp_obj["word"].as_str().unwrap_or(&word),
        resp_obj["pronunciation"]["all"].as_str().unwrap_or("")
    );
    for def in resp_obj["results"].as_array().unwrap() {
        resp = format!(
            "{} {}. {}",
            resp,
            count,
            def.as_object().unwrap()["definition"]
                .as_str()
                .unwrap()
                .replace("\n", " ")
        );
        count += 1;
    }

    resp
}

async fn cmd_thesaurus_lookup(msg: &PrivmsgMessage, command: &str) -> String {
    let word = rest_of_chat_message(msg, command).to_owned();
    tracing::info!("Thesaurus lookup: '{}'", word);
    let resp = unwrap_reqwest(explore(&word).await).await;
    if let Some(resp_obj) = resp.as_object() {
        let mut resp = format!("Alternate words for: {} ", word);

        for word_type in resp_obj.keys() {
            if let Some(word_type_list) = resp_obj[word_type].as_object() {
                resp = format!(
                    "{} => [{}s] {}",
                    resp,
                    word_type,
                    word_type_list["syn"]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|syn| syn.as_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join(", ")
                );
            }
        }

        resp
    } else {
        format!("'{}' wasn't in the thesaurus", word)
    }
}
