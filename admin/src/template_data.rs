//! Structs for the Askama template data
//!
//! Askama requires a struct to be defined for each template that takes data parameters.
//! I'm not sure if I want to keep these in a separate module like this, or if I want to have them directly with the route.
//! I figure that since the templates themselves are declared together elsewhere,
//! it sort of makes sense for these structs to be kept together as well.
//! This is one of those things that I'll work out as I make the project larger and see what sort of workflow develops
//!
//! Right now, at least, this feels sufficient and keeps like with like which makes debugging and growing the program easier.

use askama_actix::Template;

/// data available to the Index page template
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub login_url: String,
}

/// data available to the Dashboard page template
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub bot_login_url: String,
}
