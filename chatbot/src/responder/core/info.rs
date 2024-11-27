use api_consumers::time::time_at_location;
use chrono::Month;
use serde_json::Value;
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use utils::{
    message::rest_of_chat_message, num::ordinal, rand::random_from_vec, serde_json::unwrap_reqwest,
};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "core::info" {
        cmd_info()
    } else if response_fn == "core::info::time" {
        cmd_time(msg, command).await
    } else if response_fn == "core::info::discord::time" {
        cmd_discord_time(msg, command).await
    } else {
        "Unknown Function (info)".to_owned()
    }
}

fn cmd_info() -> String {
    let possible_responses = Vec::from([
        "are what happens when @TastyAndTheCats lets his cats go unsupervised",
        "are a little bit of an experiment.",
        "can't be stopped. Get on their good side by subscribing or giving bits.",
        "are taking the first steps towards complete global domination.",
    ]);

    random_from_vec(&possible_responses).unwrap().to_string()
}

async fn cmd_time(msg: &PrivmsgMessage, command: &str) -> String {
    let mut rest_of_message = rest_of_chat_message(msg, command);
    if rest_of_message.len() <= 1 {
        rest_of_message = String::from("Toronto");
    }

    let time_at_req = unwrap_reqwest(time_at_location(&rest_of_message).await).await;
    convert_time_at_location_request_to_string(time_at_req)
}

fn convert_time_at_location_request_to_string(resp: Value) -> String {
    let hour = resp["hour"].as_i64().unwrap_or(45);
    format!(
        "{} {} of {} {} at {} {} in {}",
        resp["dayOfWeek"].as_str().unwrap_or("Someday"),
        ordinal(resp["day"].as_i64().unwrap_or(32).try_into().unwrap_or(33)),
        Month::try_from(resp["month"].as_i64().unwrap_or(1).try_into().unwrap_or(1) - 1)
            .unwrap()
            .name(),
        resp["year"].as_i64().unwrap_or(5191),
        resp["time"].as_str().unwrap_or("25:69"),
        if hour > 12 {
            format!("({}:{} PM)", hour - 12, resp["minute"])
        } else {
            String::new()
        },
        format!(
            "{}{}",
            resp["timeZone"].as_str().unwrap_or("Antarctica/Troll"),
            if resp["dstActive"].as_bool().unwrap_or(false) {
                String::from(" DST")
            } else {
                String::new()
            }
        )
    )
}

async fn cmd_discord_time(msg: &PrivmsgMessage, command: &str) -> String {
    let mut rest_of_message = rest_of_chat_message(msg, command);
    if rest_of_message.len() <= 1 {
        rest_of_message = String::from("Toronto");
    }

    let time_at_req = unwrap_reqwest(time_at_location(&rest_of_message).await).await;
    convert_time_at_location_request_to_discordian_date_string(time_at_req)
}

fn convert_time_at_location_request_to_discordian_date_string(resp: Value) -> String {
    let mut lengths_of_gregorian_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut lengths_of_discordian_seasons = [73, 73, 73, 73, 73];

    let gregorian_year = resp["year"].as_i64().unwrap_or(5191);
    let is_leap_year =
        gregorian_year % 4 == 0 && (gregorian_year % 100 != 0 || gregorian_year % 400 == 0);
    let discordian_year = gregorian_year + 1166;

    if is_leap_year {
        lengths_of_gregorian_months[1] = 29;
        lengths_of_discordian_seasons[0] = 74;
    }

    let gregorian_month: usize = resp["month"].as_i64().unwrap_or(1).try_into().unwrap();
    let gregorian_day: i32 = resp["day"].as_i64().unwrap_or(1).try_into().unwrap();

    let mut day_of_year = gregorian_day;
    for i in 0..(gregorian_month - 1) {
        day_of_year = day_of_year + lengths_of_gregorian_months[i];
    }
    if is_leap_year && day_of_year == 60 {
        return format!("It's St. Tib's Day {}!", discordian_year);
    }

    let mut leap_year_offset = 0;
    if is_leap_year && day_of_year > 59 {
        leap_year_offset = -1;
    }

    let mut discordian_day = day_of_year;
    let mut discordian_season = 0;
    while discordian_day > lengths_of_discordian_seasons[discordian_season] {
        discordian_day = discordian_day - lengths_of_discordian_seasons[discordian_season];
        discordian_season = discordian_season + 1;
    }
    let discordian_day_offset: usize = ((day_of_year - 1 + leap_year_offset) % 5)
        .try_into()
        .unwrap();
    println!(
        "{} {} {} {}",
        discordian_season, discordian_day, day_of_year, leap_year_offset
    );

    let mut holidays = String::new();

    if discordian_day == 5 {
        holidays = format!(
            "{} It's the apostle holiday of {}!",
            holidays,
            ["Mungday", "Mojoday", "Syaday", "Zaraday", "Maladay"][discordian_season]
        );
    } else if discordian_day == 50 {
        holidays = format!(
            "{} It's the seasonal holiday of {}!",
            holidays,
            ["Chaoflux", "Discoflux", "Confuflux", "Bureflux", "Afflux"][discordian_season]
        );
    }

    format!(
        "It's {} the {} day of {} (year of our Goddess {}) in {}.{}",
        [
            "Sweetmorn",
            "Boomtime",
            "Pungentday",
            "Prickle-Prickle",
            "Setting Orange",
        ][discordian_day_offset],
        ordinal(discordian_day),
        [
            "Chaos",
            "Discord",
            "Confusion",
            "Bureaucracy",
            "The Aftermath",
        ][discordian_season],
        discordian_year,
        resp["timeZone"].as_str().unwrap_or("Antarctica/Troll"),
        holidays
    )
}
