use crate::{
    // auth::*,
    database::*,
    twitch::*,
};

pub fn chatbot(id: Option<i32>, name: Option<String>) -> Chatbot {
    if id.is_some() && name.is_some() {
        let name = name.unwrap();
        Chatbot {
            id: id.unwrap(),
            login: name.to_lowercase(),
            name: Some(name),
        }
    } else {
        Chatbot::default()
    }
}

pub fn app(
    id: Option<String>,
    secret: Option<String>,
    login_redirect_url: Option<String>,
    bot_login_redirect_url: Option<String>,
) -> App {
    if id.is_some() && secret.is_some() && id.is_some() && secret.is_some() {
        App {
            client_id: id.unwrap(),
            client_secret: secret.unwrap(),
            login_redirect_url: login_redirect_url.unwrap(),
            bot_login_redirect_url: bot_login_redirect_url.unwrap(),
        }
    } else {
        App::default()
    }
}

pub fn channel(id: Option<i32>, name: Option<String>) -> Channel {
    if id.is_some() && name.is_some() {
        let name = name.unwrap();
        Channel {
            id: id.unwrap(),
            login: name.to_lowercase(),
            name: Some(name),
        }
    } else {
        Channel::default()
    }
}

pub fn database(url: Option<String>) -> PostgresDatabase {
    if url.is_some() {
        PostgresDatabase { url: url.unwrap() }
    } else {
        PostgresDatabase::default()
    }
}
