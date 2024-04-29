use std::env;

use reqwest::{Client, Error, Response};

pub async fn get_video_snippet(url: &str) -> Result<Response, Error> {
    let video_id = if url.contains("youtube.com") && url.contains("v=") {
        // Long form URL
        url.split("v=").collect::<Vec<&str>>()[1]
            .split("?")
            .collect::<Vec<&str>>()[0]
    } else {
        // Shortened URL
        url.split("com/").collect::<Vec<&str>>()[1]
    };

    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KET not set");
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=snippet",
        video_id, api_key
    );
    Client::new()
        .get(url)
        .header("User-Agent", "TheCatsInChat")
        .header("Api-User-Agent", "tasty@tastyandthecats.com")
        .send()
        .await
}
