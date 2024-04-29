use reqwest::{Client, Error, Response};

use crate::openweathermap::get_lat_lng_from_location;

pub async fn time_at_location(location: &str) -> Result<Response, Error> {
    let (lat, lng) = get_lat_lng_from_location(location).await;
    time_at(lat, lng).await
}

async fn time_at(lat: String, lng: String) -> Result<Response, Error> {
    let url = format!(
        "https://timeapi.io/api/Time/current/coordinate?latitude={}&longitude={}",
        lat, lng
    );
    println!("{}", url);
    Client::new()
        .get(url)
        .header("User-Agent", "TheCatsInChat")
        .header("Api-User-Agent", "tasty@tastyandthecats.com")
        .send()
        .await
}
