use crate::{
    // auth::*,
    database::*,
    twitch::*,
};
use std::env;

pub fn chatbot(id: Option<i32>, name: Option<String>) -> Chatbot {
    let name = name.unwrap_or(env::var("DEFAULT_BOT_NAME").expect("DEFAULT_BOT_NAME is missing"));
    Chatbot {
        id: id.unwrap_or(
            env::var("DEFAULT_BOT_ID")
                .expect("DEFAULT_BOT_ID is missing")
                .parse::<i32>()
                .expect("DEFAULT_BOT_ID must be a number"),
        ),
        login: name.to_lowercase(),
        name: Some(name),
    }
}

pub fn app(
    id: Option<String>,
    secret: Option<String>,
    login_redirect_url: Option<String>,
    bot_login_redirect_url: Option<String>,
) -> App {
    App {
        client_id: id.unwrap_or(env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID is missing")),
        client_secret: secret
            .unwrap_or(env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET is missing")),
        login_redirect_url: login_redirect_url
            .unwrap_or(env::var("TWITCH_REDIRECT_URL").expect("TWITCH_REDIRECT_URL is missing")),
        bot_login_redirect_url: bot_login_redirect_url.unwrap_or(
            env::var("TWITCH_BOT_REDIRECT_URL").expect("TWITCH_BOT_REDIRECT_URL is missing"),
        ),
    }
}

pub fn channel(id: Option<i32>, name: Option<String>) -> Channel {
    let name = name.unwrap_or(env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL is missing"));

    Channel {
        id: id.unwrap_or(
            env::var("TWITCH_CHANNEL_ID")
                .expect("TWITCH_CHANNEL_ID is missing")
                .parse::<i32>()
                .expect("TWITCH_CHANNEL_ID must be a number"),
        ),
        login: name.to_lowercase(),
        name: Some(name),
    }
}

pub fn database() -> PostgresDatabase {
    PostgresDatabase {
        url: env::var("DATABASE_URL").expect("DATABASE_URL is missing"),
    }
}
