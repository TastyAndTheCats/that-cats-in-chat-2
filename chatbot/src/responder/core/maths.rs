use regex::Regex;
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use utils::{
    message::single_word_after_command,
    rand::{random_number_1_to, usize_random_number_1_to},
};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "core::maths::prime_check" {
        cmd_is_prime(msg, command)
    } else if response_fn == "core::maths::coin_toss" {
        cmd_coin_flip()
    } else if response_fn == "core::maths::roll_die" {
        cmd_roll_x_sided_die(msg, command)
    } else {
        "Unknown Function (info)".to_owned()
    }
}

fn cmd_is_prime(msg: &PrivmsgMessage, command: &str) -> String {
    let maybe_prime = single_word_after_command(msg, command)
        .parse::<usize>()
        .unwrap_or(0);

    if maybe_prime == 0 {
        return format!("Unable to compute primality of {}", single_word_after_command(msg, command));
    } else if maybe_prime == 1 || maybe_prime == 2 {
        return format!("Of course {} is prime, are you being silly?", maybe_prime);
    } else if maybe_prime % 2 == 0 {
        return format!(
            "All even numbers (like {}) are NOT prime, you don't even need to check",
            maybe_prime
        );
    };
    let mut is_prime = true;
    let mut multiple = 0;
    for check_with in (3..maybe_prime / 2).step_by(2) {
        if maybe_prime % check_with == 0 {
            multiple = check_with;
            is_prime = false;
            break;
        }
    }

    if is_prime {
        format!("{} IS prime, {{sender}}", maybe_prime)
    } else {
        format!(
            "{} is NOT prime ({} * {} = {}), {{sender}}",
            maybe_prime,
            multiple,
            maybe_prime / multiple,
            maybe_prime
        )
    }
}

fn cmd_coin_flip() -> String {
    format!(
        "{{sender}} flipped a coin and got {}",
        if random_number_1_to(2) == 1 {
            "heads"
        } else {
            "tails"
        }
    )
}

fn cmd_roll_x_sided_die(msg: &PrivmsgMessage, command: &str) -> String {
    let roll_on = single_word_after_command(msg, command);
    let re = Regex::new(r"(\d+)d(\d+)").unwrap();

    let mut number_of_dice = 1;
    let mut number_of_sides_on_die = 20;

    for (_, [num, faces]) in re.captures_iter(&roll_on).map(|c| c.extract()) {
        number_of_dice = num.parse::<usize>().unwrap_or(0);
        number_of_sides_on_die = faces.parse::<usize>().unwrap_or(0);
    }

    if number_of_dice == 0 {
        return String::from("Roll at least one die.");
    }
    if number_of_sides_on_die <= 1 {
        return String::from("Dice must have at least two faces");
    }
    if number_of_sides_on_die > std::usize::MAX
        || number_of_dice > std::usize::MAX
        || number_of_dice * number_of_sides_on_die > std::usize::MAX
    {
        return format!("Either {} is too many dice or {} is too many sides, or both, or a combination of the two", number_of_dice, number_of_sides_on_die);
    }

    let mut dice_rolls: Vec<usize> = vec![];
    for _ in 0..number_of_dice {
        dice_rolls.push(usize_random_number_1_to(
            number_of_sides_on_die.try_into().unwrap_or(0),
        ));
    }

    let total: usize = dice_rolls.iter().sum();

    format!(
        "{{sender}} rolled {} {}-sided {} and got {}{}",
        number_of_dice,
        number_of_sides_on_die,
        if number_of_dice == 1 { "die" } else { "dice" },
        total,
        if dice_rolls.len() > 1 {
            format!(
                " ({})",
                dice_rolls
                    .into_iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<String>>()
                    .join("+")
            )
        } else {
            String::new()
        },
    )
}
