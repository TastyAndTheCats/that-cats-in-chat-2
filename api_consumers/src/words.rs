use std::env;

use reqwest::{Client, Error, Response};

pub async fn explore(word: &str) -> Result<Response, Error> {
    let api_key = env::var("THESAURUS_API_KEY").expect("THESAURUS_API_KEY isn't set");
    let url = format!(
        "http://words.bighugelabs.com/api/2/{}/{}/json",
        api_key, word
    );
    Client::new()
        .get(url)
        .header("User-Agent", "TheCatsInChat")
        .header("Api-User-Agent", "tasty@tastyandthecats.com")
        .send()
        .await
}

pub async fn define(word: &str) -> Result<Response, Error> {
    let url = format!("https://wordsapiv1.p.rapidapi.com/words/{}", word);
    let api_key = env::var("DICTIONARY_API_KEY").expect("DICTIONARY_API_KEY isn't set");
    Client::new()
        .get(url)
        .header("User-Agent", "TheCatsInChat")
        .header("Api-User-Agent", "tasty@tastyandthecats.com")
        .header("x-rapidapi-key", api_key)
        .header("x-rapidapi-host", "wordsapiv1.p.rapidapi.com")
        .send()
        .await
}
