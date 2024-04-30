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
    if response_fn.starts_with("core::facts::advice") {
        format!("Advice for {{sender}}: {}", x_facts("advice.txt"))
    } else if response_fn.starts_with("core::facts::catfact") {
        format!("Cat fact for {{sender}}: {}", x_facts("catfacts.txt"))
    } else if response_fn.starts_with("core::facts::chucknorris") {
        format!(
            "Chuck Norris Fact for {{sender}}: {}",
            x_facts("chuckfacts.txt")
        )
    } else if response_fn.starts_with("core::facts::dadjoke") {
        format!("Dad joke for {{sender}}: {}", x_facts("dadjokes.txt"))
    } else if response_fn.starts_with("core::facts::discord") {
        format!(
            "Discordianism-ism for {{sender}}: {}",
            x_facts("discord.txt")
        )
    } else if response_fn.starts_with("core::facts::dogfact") {
        format!("Dog fact for {{sender}}: {}", x_facts("dogfacts.txt"))
    } else if response_fn.starts_with("core::facts::fight") {
        format!("Hey, buddy {{sender}}: {}", x_facts("fight.txt"))
    } else if response_fn.starts_with("core::facts::fortune_cookie") {
        format!("Fortune for {{sender}}: {}", x_facts("fortune_cookies.txt"))
    } else if response_fn.starts_with("core::facts::numfact") {
        format!("Number fact for {{sender}}: {}", x_facts("numfacts.txt"))
    } else if response_fn.starts_with("core::facts::rickyism") {
        format!("Rickyism for {{sender}}: {}", x_facts("rickyisms.txt"))
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
    // tracing::debug!("filepath: {}", filepath);
    let facts = load_lines_from_file(filepath);
    let facts_length = facts.len().try_into().unwrap_or(0);
    let fact_choice: usize = random_number_0_to(facts_length).try_into().unwrap();
    format!("{} (#{})", facts[fact_choice].to_string(), fact_choice + 1)
}
