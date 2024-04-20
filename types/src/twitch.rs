/// From Environment Variables
#[derive(Debug)]
pub struct Channel {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

/// From Environment Variables
#[derive(Debug)]
pub struct Chatbot {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

/// From Environment Variables
#[derive(Debug)]
pub struct App {
    pub client_id: String,
    pub client_secret: String,
    pub login_redirect_url: String,
    pub bot_login_redirect_url: String,
}
