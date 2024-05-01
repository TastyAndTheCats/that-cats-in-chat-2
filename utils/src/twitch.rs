use std::env;

use database::bot::get_access_token;

pub fn client_and_access_token() -> (String, Option<String>) {
    (
        env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable."),
        get_access_token(Some(
            env::var("DEFAULT_BOT_ID")
                .expect("Missing DEFAULT_BOT_ID environment variable.")
                .parse::<i32>()
                .unwrap(),
        ))
        .unwrap_or(get_access_token(None).unwrap())
        .access_token,
    )
}
