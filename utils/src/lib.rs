//! Contains some helpful functions that made me go cross-eyed when they were left in place
//! Most of these modules are representative of a specific third-party crate (rand, url, etc.)
//! Functions in the lib file are data conversions

use twitch_irc::message::PrivmsgMessage;

pub mod rand;
pub mod url;

/// Parses a String as an i32
/// This is kind of stupid because it's so short but it's annoying to write.
pub fn parse_id(id: String) -> i32 {
    id.parse::<i32>().unwrap().to_owned()
}

/// Returns the portion of the message after the command
pub fn rest_of_chat_message(msg: &PrivmsgMessage, command: &str) -> String {
    let rest_of_message = msg.message_text.split(command).collect::<Vec<&str>>()[1..].join("");
    rest_of_message.trim().to_owned()
}

/// Returns the first word after the command (or "tastyandthecats")
pub fn single_word_after_command(msg: &PrivmsgMessage, command: &str) -> String {
    let rest_of_message = rest_of_chat_message(msg, command);

    if rest_of_message.len() > 0 {
        if rest_of_message.contains(" ") {
            return rest_of_message.split(" ").collect::<Vec<&str>>()[0].to_owned();
        } else {
            return rest_of_message.to_owned();
        }
    } else {
        return "tastyandthecats".to_owned();
    }
}
