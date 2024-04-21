use std::env;

pub struct PostgresDatabase {
    username: String,
    password: String,
    database: String,
    url: String,
}

pub struct TwitchChannel {
    id: i32,
    login: String,
    name: Option<String>,
}

const POSTGRES_USER: String =
    env::var("POSTGRES_USER").expect("Missing POSTGRES_USER environment variable.");
const POSTGRES_DB: String =
    env::var("POSTGRES_DB").expect("Missing POSTGRES_DB environment variable.");
const POSTGRES_PASSWORD: String =
    env::var("POSTGRES_PASSWORD").expect("Missing POSTGRES_PASSWORD environment variable.");
const DATABASE_URL: String =
    env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable.");
const TWITCH_CHANNEL_ID: i32 = env::var("TWITCH_CHANNEL_ID")
    .expect("Missing TWITCH_CHANNEL_ID environment variable.")
    .parse::<i32>()
    .expect("TWITCH_CHANNEL_ID was not a number");
const TWITCH_CHANNEL: String =
    env::var("TWITCH_CHANNEL").expect("Missing TWITCH_CHANNEL environment variable.");
const TWITCH_CLIENT_ID: String =
    env::var("TWITCH_CLIENT_ID").expect("Missing TWITCH_CLIENT_ID environment variable.");
const TWITCH_CLIENT_SECRET: String =
    env::var("TWITCH_CLIENT_SECRET").expect("Missing TWITCH_CLIENT_SECRET environment variable.");
const MY_LOCAL_DOMAIN: String =
    env::var("MY_LOCAL_DOMAIN").expect("Missing MY_LOCAL_DOMAIN environment variable.");
const TWITCH_REDIRECT_URL: String =
    env::var("TWITCH_REDIRECT_URL").expect("Missing TWITCH_REDIRECT_URL environment variable.");
const TWITCH_BOT_REDIRECT_URL: String = env::var("TWITCH_BOT_REDIRECT_URL")
    .expect("Missing TWITCH_BOT_REDIRECT_URL environment variable.");
const DEFAULT_BOT_NAME: String =
    env::var("DEFAULT_BOT_NAME").expect("Missing DEFAULT_BOT_NAME environment variable.");
const DEFAULT_BOT_ID: i32 = env::var("DEFAULT_BOT_ID")
    .expect("Missing DEFAULT_BOT_ID environment variable.")
    .parse::<i32>()
    .expect("DEFAULT_BOT_ID was not a number");
