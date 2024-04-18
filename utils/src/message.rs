use twitch_irc::message::PrivmsgMessage;

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
