use reqwest::{Client, Error, Response};

pub async fn free_games() -> Result<Response, Error> {
    Client::new()
        .get("https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US&country=US")
        .header("User-Agent", "TheCatsInChat")
        .header("Api-User-Agent", "tasty@tastyandthecats.com")
        .send()
        .await
}
