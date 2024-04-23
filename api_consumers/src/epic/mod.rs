use reqwest::{Client, Error, Response};

pub async fn free_games() -> Result<Response, Error> {
    Client::new()
        .get("https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US&country=US")
        .send()
        .await
}
