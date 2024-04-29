use twitch_irc::message::PrivmsgMessage;

use utils::message::single_word_after_command;

use super::object::{Emoji, EmojiProperties};

pub fn cheese(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "wedges of delicious British Cheddar",
            "chunks of crumbly Red Leicester",
            "crumbles of delicious salty feta",
        ],
        vec!["🧀"],
        Some(2),
        Some(4),
        None,
        None,
        None,
    );
    emo.message(msg, command)
}

pub fn butter(msg: &PrivmsgMessage, command: &str) -> String {
    let recipient = single_word_after_command(msg, command);
    let emo = Emoji::new(
        vec!["stick of butter", "pound of butter", "tablespoon of butter"],
        vec!["🧈"],
        Some(1),
        Some(1),
        None,
        Some(vec![
            "and it's already starting to melt",
            "from their pocket",
            &format!("they pulled out from behind {}'s ear", recipient),
        ]),
        Some(vec!["That's kind of gross.", "", ""]),
    );
    emo.message(msg, command)
}

pub fn ice_cream(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["bowl or cone of ice cream"],
        vec!["🍨", "🍦"],
        Some(1),
        Some(1),
        None,
        Some(vec!["⬅️ depending on which emoji showed up"]),
        None,
    );
    emo.message(msg, command)
}
