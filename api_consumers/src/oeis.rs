use reqwest::{Client, Error, Response};
use utils::rand::random_number_1_to;

pub async fn random_sequence() -> Result<Response, Error> {
    let sequence = random_number_1_to(370000);
    get_sequence(sequence).await
}

pub async fn get_sequence(sequence: i32) -> Result<Response, Error> {
    let url = format!("https://oeis.org/search?fmt=json&q=id:A{}", sequence);
    Client::new().get(url).send().await
}
