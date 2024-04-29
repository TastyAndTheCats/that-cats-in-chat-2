use twitch_irc::message::PrivmsgMessage;

use super::object::{Emoji, EmojiProperties};

pub fn grapes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["grapes"],
        vec!["🍇"],
        Some(5),
        Some(15),
        None,
        Some(vec!["", "in a bag", "in a bunch", "in a bowl"]),
        Some(vec!["Mmmm.. juicy!", "... but they are rotten"]),
    );
    emo.message(msg, command)
}

pub fn melons(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["melons", "honeydews", "canteloupes"],
        vec!["🍈"],
        Some(2),
        Some(3),
        None,
        None,
        None,
    );
    emo.message(msg, command)
}

pub fn watermelons(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["watermelon slices"],
        vec!["🍉"],
        Some(2),
        Some(5),
        None,
        None,
        Some(vec!["They're so juicy and fresh!", "", ""]),
    );
    emo.message(msg, command)
}

pub fn tangerines(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["tangerines"],
        vec!["🍊"],
        Some(2),
        Some(5),
        None,
        None,
        Some(vec!["They're so juicy and fresh!", "", ""]),
    );
    emo.message(msg, command)
}

pub fn oranges(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["oranges"],
        vec!["🍊"],
        Some(6),
        Some(16),
        None,
        None,
        Some(vec!["They're so juicy and fresh!", "", ""]),
    );
    emo.message(msg, command)
}

pub fn lemons(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["lemons"],
        vec!["🍋"],
        Some(2),
        Some(50),
        None,
        None,
        Some(vec!["Make lemonade!", "You're so sour!"]),
    );
    emo.message(msg, command)
}

pub fn bananas(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["bananas"],
        vec!["🍌"],
        Some(3),
        Some(12),
        None,
        None,
        Some(vec![
            "Tey're just starting to turn brown.",
            "They're still green.",
            "These are perfectly ripe!",
            "These need to go into the freezer before they go even more brown.",
        ]),
    );
    emo.message(msg, command)
}

pub fn pineapple(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["pineapple"],
        vec!["🍍"],
        None,
        None,
        None,
        Some(vec!["in a bowl", "in a bag", "cut into pieces"]),
        None,
    );
    emo.message(msg, command)
}

pub fn mangoes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["mangoes"],
        vec!["🥭"],
        Some(6),
        Some(24),
        None,
        Some(vec![
            "in a bowl",
            "in a bag",
            "fresh from the tree",
            "cut into pieces",
        ]),
        None,
    );
    emo.message(msg, command)
}

pub fn apples(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "Red Delicious apples",
            "Golden Delicious apples",
            "McIntosh apples",
            "Empire apples",
            "Jazz apples",
            "Gala apples",
            "Seneca Crisp apples",
            "Paulared apples",
            "Zestar apples",
            "Cripps Pink apples",
            "Jonagold apples",
            "Opal apples",
            "Honeycrisp apples",
            "Granny Smith apples",
            "Fuji apples",
            "Cortland apples",
            "Macoun apples",
            "Braeburn apples",
        ],
        vec!["🍎", "🍏"],
        Some(5),
        Some(18),
        None,
        Some(vec![
            "in a bowl",
            "in a bag",
            "fresh from the tree",
            "cut into pieces",
            "in a burlap sack",
        ]),
        Some(vec!["", "So crunchy!"]),
    );
    emo.message(msg, command)
}

pub fn pears(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["pears"],
        vec!["🍐"],
        Some(2),
        Some(8),
        None,
        Some(vec![
            "in a bowl",
            "in a bag",
            "fresh from the tree",
            "cut into pieces",
        ]),
        None,
    );
    emo.message(msg, command)
}

pub fn peaches(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["peaches"],
        vec!["🍑"],
        Some(3),
        Some(7),
        None,
        Some(vec![
            "in a bowl",
            "in a bag",
            "fresh from the tree",
            "cut into pieces",
        ]),
        None,
    );
    emo.message(msg, command)
}

pub fn cherries(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["cherries"],
        vec!["🍒"],
        Some(6),
        Some(29),
        None,
        Some(vec![
            "in a bowl",
            "in a bag",
            "fresh from the tree",
            "cut into pieces",
        ]),
        None,
    );
    emo.message(msg, command)
}

pub fn strawberries(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["strawberries"],
        vec!["🍓"],
        Some(4),
        Some(46),
        None,
        Some(vec!["with whipped cream", "dipped in chocolate"]),
        None,
    );
    emo.message(msg, command)
}

pub fn kiwis(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["kiwifruit (or just kiwi)"],
        vec!["🥝"],
        Some(4),
        Some(12),
        None,
        Some(vec![
            "which are very ripe",
            "which are not ready to eat yet",
        ]),
        Some(vec![
            "Peel them with a spoon!",
            "The skin is edible, you know?",
        ]),
    );
    emo.message(msg, command)
}

pub fn tomatoes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["tomatoes"],
        vec!["🍅"],
        Some(6),
        Some(24),
        Some(vec![&format!("{} throws", msg.sender.name)]),
        None,
        Some(vec!["Dodge, duck, dip, dive, and dodge!"]),
    );
    emo.message(msg, command)
}

pub fn coconuts(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["coconut halves"],
        vec!["🥥"],
        Some(2),
        Some(16),
        None,
        Some(vec!["(for making horse noises)"]),
        Some(vec!["Are you suggesting that coconuts migrate?"]),
    );
    emo.message(msg, command)
}

pub fn avocados(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["avocado halves"],
        vec!["🥑"],
        None,
        None,
        None,
        Some(vec!["with !salt and pepper"]),
        Some(vec![
            "Guacamole time!",
            "Do you have any hard-boiled !eggs?",
        ]),
    );
    emo.message(msg, command)
}

pub fn hot_peppers(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "jalapeño",
            "habanero",
            "ghost",
            "trinidad scorpion",
            "carolina reaper",
            "naga viper",
            "dorset naga",
        ],
        vec!["🌶️"],
        None,
        None,
        None,
        Some(vec!["peppers"]),
        Some(vec!["Those are HOT!", "Not so spicy!"]),
    );
    emo.message(msg, command)
}

pub fn cucumber(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["cucumber"],
        vec!["🥒"],
        None,
        None,
        None,
        None,
        Some(vec!["", "", "", "It's the second-most phallic vegetable."]),
    );
    emo.message(msg, command)
}

pub fn jam(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "jar of 🍇 jelly",
            "jar of 🥭 jam",
            "jar of 🍎 jelly",
            "jar of 🍐 jelly",
            "jar of 🍑 jam",
            "jar of 🍒 jam",
        ],
        vec![],
        None,
        None,
        None,
        None,
        None,
    );
    emo.message(msg, command)
}
