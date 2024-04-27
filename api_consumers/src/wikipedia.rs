use reqwest::{Client, Error, Response};
use utils::{message::truncate_response_for_twitch, serde_json::unwrap_reqwest};

pub async fn lookup(title: String) -> String {
    let page = unwrap_reqwest(get_wiki(title).await).await;
    let page_obj = page.as_object().unwrap();

    tracing::debug!("{:?}", page_obj);

    let resp_title = page_obj["title"].as_str().unwrap_or("Not found.");
    let extract = if resp_title != "Not found." {
        page_obj["extract"]
            .as_str()
            .unwrap_or("Invalid lookup")
            .replace("\n", " ")
    } else {
        String::from(resp_title)
    };

    truncate_response_for_twitch(extract)
}

pub async fn get_wiki(title: String) -> Result<Response, Error> {
    let url = if title.len() > 0 {
        format!(
            "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
            title
        )
    } else {
        String::from("https://en.wikipedia.org/api/rest_v1/page/random/summary")
    };
    tracing::debug!("looking up {}", url);
    Client::new()
        .get(url)
        .header("User-Agent", "TheCatsInChat")
        .header("Api-User-Agent", "tasty@tastyandthecats.com")
        .send()
        .await
}
