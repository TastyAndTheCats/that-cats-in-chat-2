use std::env;

use twitch_irc::message::PrivmsgMessage;

use api_consumers::oeis::{get_sequence, random_sequence};
use database::models::responders::TwitchResponder;
use utils::{
    file::load_lines_from_file, message::first_word_after_command_as_number,
    rand::random_number_0_to, serde_json::unwrap_reqwest,
};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("core::facts::advice") {
        return cmd_advice();
    } else if response_fn.starts_with("core::facts::catfact") {
        return cmd_catfact();
    } else if response_fn.starts_with("core::facts::chucknorris") {
        return cmd_chuckfact();
    } else if response_fn.starts_with("core::facts::dadjoke") {
        return cmd_dadjoke();
    } else if response_fn.starts_with("core::facts::dogfact") {
        return cmd_dogfact();
    } else if response_fn.starts_with("core::facts::fortune_cookie") {
        return cmd_fortunecookies();
    } else if response_fn.starts_with("core::facts::numfact") {
        return cmd_numfact();
    } else if response_fn.starts_with("core::facts::oeis") {
        return cmd_oeis(msg, command).await;
    } else if response_fn.starts_with("core::facts::rickyism") {
        return cmd_rickyism();
    } else {
        return "Unknown Function (facts)".to_owned();
    }
}

fn cmd_advice() -> String {
    format!("Advice for {{sender}}: {}", x_facts("advice.txt"))
}

fn cmd_catfact() -> String {
    format!("Cat fact for {{sender}}: {}", x_facts("catfacts.txt"))
}

fn cmd_chuckfact() -> String {
    format!(
        "Chuck Norris Fact for {{sender}}: {}",
        x_facts("chuckfacts.txt")
    )
}

fn cmd_dadjoke() -> String {
    format!("Dad joke for {{sender}}: {}", x_facts("dadjokes.txt"))
}

fn cmd_dogfact() -> String {
    format!("Dog fact for {{sender}}: {}", x_facts("dogfacts.txt"))
}

fn cmd_fortunecookies() -> String {
    format!("Fortune for {{sender}}: {}", x_facts("fortune_cookies.txt"))
}

fn cmd_numfact() -> String {
    format!("Number fact for {{sender}}: {}", x_facts("numfacts.txt"))
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

fn cmd_rickyism() -> String {
    format!("Rickyism for {{sender}}: {}", x_facts("rickyisms.txt"))
}

fn x_facts(filename: &str) -> String {
    let filepath = format!(
        "{}/chatbot/src/responder/core/facts/{}",
        env::current_dir().unwrap().display(),
        filename
    );
    // println!("filepath: {}", filepath);
    let facts = load_lines_from_file(filepath);
    let facts_length = facts.len().try_into().unwrap_or(0);
    let fact_choice: usize = random_number_0_to(facts_length).try_into().unwrap();
    format!("{} (#{})", facts[fact_choice].to_string(), fact_choice + 1)
}
