use std::env;

/// From Environment Variables
#[derive(Debug)]
pub struct Channel {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

impl Default for Channel {
    fn default() -> Self {
        let name = env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL is missing");
        Channel {
            id: env::var("TWITCH_CHANNEL_ID")
                .expect("TWITCH_CHANNEL_ID is missing")
                .parse::<i32>()
                .expect("TWITCH_CHANNEL_ID must be a number"),
            login: name.to_lowercase(),
            name: Some(name),
        }
    }
}

/// From Environment Variables
#[derive(Debug)]
pub struct Chatbot {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

impl Default for Chatbot {
    fn default() -> Self {
        let name = env::var("DEFAULT_BOT_NAME").expect("DEFAULT_BOT_NAME is missing");
        Chatbot {
            id: env::var("DEFAULT_BOT_ID")
                .expect("DEFAULT_BOT_ID is missing")
                .parse::<i32>()
                .expect("DEFAULT_BOT_ID must be a number"),

            login: name.to_lowercase(),
            name: Some(name),
        }
    }
}

/// From Environment Variables
#[derive(Debug)]
pub struct App {
    pub client_id: String,
    pub client_secret: String,
    pub login_redirect_url: String,
    pub bot_login_redirect_url: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            client_id: env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID is missing"),
            client_secret: env::var("TWITCH_CLIENT_SECRET")
                .expect("TWITCH_CLIENT_SECRET is missing"),
            login_redirect_url: env::var("TWITCH_REDIRECT_URL")
                .expect("TWITCH_REDIRECT_URL is missing"),
            bot_login_redirect_url: env::var("TWITCH_BOT_REDIRECT_URL")
                .expect("TWITCH_BOT_REDIRECT_URL is missing"),
        }
    }
}
