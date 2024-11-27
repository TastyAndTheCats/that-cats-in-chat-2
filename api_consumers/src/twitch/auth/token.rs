use database::login::user::{get_refresh_token, update_refresh_token};
use reqwest::Client;
use types::get;
use utils::{serde_json::unwrap_reqwest, url::compose_post_body};

/// Refresh the Twitch access tokens for a user and save them to the db
/// If we successfully refresh the token we'll return it
pub async fn refresh(user_id: i32) -> Option<String> {
    let channel = get::channel(Some(user_id), None);
    match get_current_refresh_token(channel.id) {
        Some(refresh_token) => {
            let (access_token, refresh_token) =
                get_new_access_and_refresh_tokens(refresh_token).await;
            save_new_access_token(channel.id, &access_token, &refresh_token);
            tracing::debug!(
                "new stuff: access: {}, refresh: {}",
                access_token,
                refresh_token
            );
            Some(refresh_token)
        }
        None => None,
    }
}

/// Assume this works
fn get_current_refresh_token(channel_id: i32) -> Option<String> {
    get_refresh_token(channel_id).unwrap_or(None)
}

/// This is right out of the Twitch documentation
async fn get_new_access_and_refresh_tokens(refresh_token: String) -> (String, String) {
    let app = get::app(None, None, None, None);
    let params: Vec<(&str, String)> = vec![
        ("client_id", app.client_id),
        ("client_secret", app.client_secret),
        ("grant_type", "refresh_token".to_owned()),
        ("refresh_token", refresh_token),
    ];
    // NOTE: this is the reqwest that runs refresh_token so can't be replaced with twitch::{get,post}, nor should it be;
    let resp = unwrap_reqwest(
        Client::new()
            .post("https://id.twitch.tv/oauth2/token")
            .body(compose_post_body(&params))
            .send()
            .await,
    )
    .await;

    let refresh_resp = resp.as_object().unwrap();
    let access_token = refresh_resp["access_token"].as_str().unwrap_or("");
    let refresh_token = refresh_resp["refresh_token"].as_str().unwrap_or("");

    (access_token.to_owned(), refresh_token.to_owned())
}

/// Wrapping database calls like this is a good way to increase your maintenance overhead
/// but I'm still gonna do it as long as it's not a pub fn since "organization"
fn save_new_access_token(channel_id: i32, access_token: &str, refresh_token: &str) {
    update_refresh_token(channel_id, access_token, refresh_token);
}
