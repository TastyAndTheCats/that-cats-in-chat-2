use api_consumers::openweathermap::get_weather_at;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde_json::Value;
use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;
use utils::{
    message::rest_of_chat_message, metric::convert_celcius_to_fahrenheit,
    serde_json::unwrap_reqwest,
};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "api::openweathermap::weather" {
        cmd_weather(msg, command).await
    } else if response_fn == "api::openweathermap::weather_full" {
        cmd_weather_full(msg, command).await
    } else {
        "Unknown Function (openweathermap)".to_owned()
    }
}

async fn cmd_weather(msg: &PrivmsgMessage, command: &str) -> String {
    let rest_of_message = rest_of_chat_message(msg, command);
    let weather_json = unwrap_reqwest(get_weather_at(&rest_of_message).await).await;
    // println!("weather_json: {:?}", weather_json);
    cmd_weather_short(&rest_of_message, &weather_json)
}

async fn cmd_weather_full(msg: &PrivmsgMessage, command: &str) -> String {
    let rest_of_message = rest_of_chat_message(msg, command);
    let weather_json = unwrap_reqwest(get_weather_at(&rest_of_message).await).await;
    cmd_weather_long(&rest_of_message, &weather_json)
}

fn cmd_weather_short(rest_of_message: &str, weather_json: &Value) -> String {
    let name = weather_json["name"].as_str().unwrap_or(rest_of_message);
    let sys = weather_json["sys"].as_object().unwrap();
    let fake_country_code = Value::String("ZZ".to_owned());
    let country = sys
        .get("country")
        .unwrap_or(&fake_country_code)
        .as_str()
        .unwrap_or("");

    let mut response = Vec::from(["In".to_owned()]);
    response.push(format!("{}, {}", name, country));

    match weather_json["weather"].as_array() {
        Some(weather) => {
            response.push(format!(
                "the forecast calls for {}.",
                weather[0]["description"].as_str().unwrap_or("")
            ));
        }
        None => {}
    }

    match weather_json["main"].as_object() {
        Some(main) => {
            let temp = main["temp"].as_f64().unwrap_or(0.0);
            response.push(format!(
                "Current temperature is {:.1}°C/{:.1}°F,",
                temp,
                convert_celcius_to_fahrenheit(temp)
            ));
            let feels_like = main["feels_like"].as_f64().unwrap_or(0.0);
            response.push(format!(
                "but it feels like {:.1}°C/{:.1}°F.",
                feels_like,
                convert_celcius_to_fahrenheit(feels_like)
            ));
            let temp_min = main["temp_min"].as_f64().unwrap_or(0.0);
            response.push(format!(
                "With a low of {:.1}°C/{:.1}°F",
                temp_min,
                convert_celcius_to_fahrenheit(temp_min)
            ));
            let temp_max = main["temp_max"].as_f64().unwrap_or(0.0);
            response.push(format!(
                "and a high of {:.1}°C/{:.1}°F",
                temp_max,
                convert_celcius_to_fahrenheit(temp_max)
            ));
        }
        None => {}
    }

    response.join(" ")
}

fn cmd_weather_long(rest_of_message: &str, weather_json: &Value) -> String {
    let simple_weather = cmd_weather_short(rest_of_message, weather_json);
    let mut response = Vec::from([simple_weather]);

    match weather_json["main"].as_object() {
        Some(main) => {
            response.push(format!("{}% humidity", main["humidity"]));
            response.push(format!("{}hPa", main["pressure"]));
        }
        None => {}
    }

    match weather_json["wind"].as_object() {
        Some(wind) => {
            response.push(format!(
                "Wind: {}kph {}",
                wind["speed"],
                degrees_to_cardinal_direction(wind["deg"].as_f64().unwrap_or(0.0))
            ));
            match wind.get("gust") {
                Some(gust) => response.push(format!("({}kph gusts)", gust)),
                None => {}
            }
        }
        None => {}
    }

    match weather_json["clouds"].as_object() {
        Some(clouds) => {
            response.push(format!("{}% cloud coverage", clouds["all"]));
        }
        None => {}
    }

    match weather_json["sys"].as_object() {
        Some(sys) => {
            let offset = weather_json["timezone"].as_i64().unwrap_or(0);
            let sunrise =
                convert_timestamp_to_timezone(sys["sunrise"].as_i64().unwrap_or(0), offset);
            let sunset = convert_timestamp_to_timezone(sys["sunset"].as_i64().unwrap_or(0), offset);
            response.push(format!("Sunrise: {}", sunrise.format("%H:%M")));
            response.push(format!("Sunrise: {}", sunset.format("%H:%M")));
        }
        None => {}
    }

    response.join(", ")
}

fn degrees_to_cardinal_direction(degrees: f64) -> &'static str {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW", "N"];
    let index = ((degrees + 22.5) / 45.0).floor() as usize % 8;
    directions[index]
}

fn convert_timestamp_to_timezone(timestamp: i64, timezone_offset: i64) -> DateTime<FixedOffset> {
    // Convert the naive datetime to the specified time zone
    DateTime::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp(timestamp, 0),
        FixedOffset::east_opt(timezone_offset as i32).unwrap(),
    )
}
