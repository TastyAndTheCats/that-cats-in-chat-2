use std::env;

use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use utils::{file::load_lines_from_file, rand::random_number_0_to};

pub async fn dispatch(
    responder: &TwitchResponder,
    _msg: &PrivmsgMessage,
    _command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::facts::catfact") {
        return cmd_catfact();
    } else if response_fn.starts_with("core::facts::dogfact") {
        return cmd_dogfact();
    } else {
        return "Unknown Function (facts)".to_owned();
    }
}

fn x_facts(filename: &str) -> String {
    let filepath = format!(
        "{}/chatbot/src/responder/core/facts/{}",
        env::current_dir().unwrap().display(),
        filename
    );
    println!("filepath: {}", filepath);
    let facts = load_lines_from_file(filepath);
    let facts_length = facts.len().try_into().unwrap_or(0);
    let fact_choice: usize = random_number_0_to(facts_length).try_into().unwrap();
    format!("{} (#{})", facts[fact_choice].to_string(), fact_choice + 1)
}

fn cmd_dogfact() -> String {
    format!("Dog fact for {{sender}}: {}", x_facts("dogfacts.txt"))
}

fn cmd_catfact() -> String {
    format!("Cat fact for {{sender}}: {}", x_facts("catfacts.txt"))
}
