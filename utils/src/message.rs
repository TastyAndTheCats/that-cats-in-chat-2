use std::num::ParseIntError;

use twitch_irc::message::PrivmsgMessage;

/// Returns the portion of the message after the command
pub fn rest_of_chat_message(msg: &PrivmsgMessage, command: &str) -> String {
    msg.message_text.split(command).collect::<Vec<&str>>()[1..]
        .join("")
        .trim()
        .to_owned()
}

/// Returns the first word after the command (or the message sender's own name)
pub fn single_word_after_command(msg: &PrivmsgMessage, command: &str) -> String {
    let rest_of_message = rest_of_chat_message(msg, command);

    if rest_of_message.len() > 0 {
        if rest_of_message.contains(" ") {
            rest_of_message.split(" ").collect::<Vec<&str>>()[0].to_owned()
        } else {
            rest_of_message.to_owned()
        }
    } else {
        msg.sender.name.to_owned()
    }
}

// Returns the first word but as a number (or -1)
pub fn first_word_after_command_as_number(msg: &PrivmsgMessage, command: &str) -> Result<i32, ParseIntError> {
    single_word_after_command(msg, command)
        .parse::<i32>()
}
