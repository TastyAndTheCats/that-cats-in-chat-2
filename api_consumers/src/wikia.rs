use reqwest::{Client, Error, Response};

pub async fn lookup(subdomain: &str, topic: &str) -> Result<Response, Error> {
    let url = format!("https://en.wikipedia.org/w/rest.php/v1/page/Main_Page/history");
    Client::new().get(url).send().await
}
