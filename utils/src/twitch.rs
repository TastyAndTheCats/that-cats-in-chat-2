use std::env;

use database::twitch_access_token;

pub fn client_and_access_token() -> (String, String) {
    let client_id =
        env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable.");
    let access_token = twitch_access_token().unwrap();
    (client_id, access_token)
}
