use std::cmp::max;

use twitch_irc::message::PrivmsgMessage;

use utils::{
    message::single_word_after_command,
    rand::{random_from_vec, random_number_x_to_y},
};

pub struct Emoji {
    min: i32,
    max: i32,
    symbols: Vec<String>,
    names: Vec<String>,
    befores: Vec<String>,
    descriptions: Vec<String>,
    followups: Vec<String>,
}

pub trait EmojiProperties {
    fn new(
        names: Vec<&str>,
        symbols: Vec<&str>,
        min: Option<i32>,
        max: Option<i32>,
        befores: Option<Vec<&str>>,
        descriptions: Option<Vec<&str>>,
        followups: Option<Vec<&str>>,
    ) -> Self;
    fn message(&self, msg: &PrivmsgMessage, command: &str) -> String;
}

impl EmojiProperties for Emoji {
    fn new(
        names: Vec<&str>,
        symbols: Vec<&str>,
        minimum: Option<i32>,
        maximum: Option<i32>,
        befores: Option<Vec<&str>>,
        descriptions: Option<Vec<&str>>,
        followups: Option<Vec<&str>>,
    ) -> Self {
        let minimum = minimum.unwrap_or(1);
        Emoji {
            names: names.into_iter().map(|n| n.to_owned()).collect(),
            symbols: symbols.into_iter().map(|n| n.to_owned()).collect(),
            min: minimum,
            max: max(maximum.unwrap_or(1), minimum),
            befores: befores
                .unwrap_or(vec![])
                .into_iter()
                .map(|n| n.to_owned())
                .collect(),
            descriptions: descriptions
                .unwrap_or(vec![])
                .into_iter()
                .map(|n| n.to_owned())
                .collect(),
            followups: followups
                .unwrap_or(vec![])
                .into_iter()
                .map(|n| n.to_owned())
                .collect(),
        }
    }
    fn message(&self, msg: &PrivmsgMessage, command: &str) -> String {
        let mut recipient = single_word_after_command(msg, command);
        if recipient == msg.sender.name {
            recipient = "themself".to_string();
        }
        let count = random_number_x_to_y(self.min, self.max);
        let symbols = random_from_vec(&self.symbols)
            .unwrap_or(&String::new())
            .repeat(count.try_into().unwrap_or(1));

        let mut before = format!("{} gives", msg.sender.name);
        if let Some(d) = random_from_vec(&self.befores) {
            before = format!("{}", d);
        }

        let mut description = String::new();
        if let Some(d) = random_from_vec(&self.descriptions) {
            description = format!(" {}", d);
        }

        let mut followup = String::new();
        if let Some(f) = random_from_vec(&self.followups) {
            followup = format!(" {}", f);
        }

        format!(
            "{} {} {} {}{} to {}.{}",
            before,
            if count == 1 {
                "a".to_owned()
            } else {
                format!("{}", count)
            },
            random_from_vec(&self.names).unwrap_or(&String::new()),
            symbols,
            description,
            recipient,
            followup
        )
    }
}
