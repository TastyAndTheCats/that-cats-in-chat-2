use std::env;

use reqwest::{Client, Error, Response};
use utils::serde_json::unwrap_reqwest;

pub async fn get_weather_at(location: &str) -> Result<Response, Error> {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");
    let (lat, lng) = get_lat_lng_from_location(location).await;
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?units=metric&lat={}&lon={}&appid={}",
        lat, lng, api_key
    );
    Client::new().get(url).send().await
}

async fn get_lat_lng_from_location(location: &str) -> (String, String) {
    let geocoding = unwrap_reqwest(geocoding(location).await).await;
    let loc = &geocoding[0];
    (
        loc["lat"].as_f64().unwrap_or(0.0).to_string(),
        loc["lon"].as_f64().unwrap_or(0.0).to_string(),
    )
}

async fn geocoding(location: &str) -> Result<Response, Error> {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");
    let location = separate_location_into_constituent_parts(location).join(",");
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
        location, api_key
    );
    Client::new().get(url).send().await
}

fn separate_location_into_constituent_parts(location: &str) -> Vec<&str> {
    let mut location_split: Vec<&str> = location
        .split(",")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();
    let mut loc = Vec::new();
    loc.push(if location_split.len() > 0 {
        location_split.remove(0)
    } else {
        ""
    }); // city
    loc.push(if location_split.len() > 0 {
        &location_split.remove(0)[..2]
    } else {
        ""
    }); // state
    loc.push(if location_split.len() > 0 {
        &location_split.remove(0)[..2]
    } else {
        ""
    }); // country

    if loc == Vec::from(["", "", ""]) {
        Vec::from(["Toronto", "ON", "CA"])
    } else {
        loc
    }
}
