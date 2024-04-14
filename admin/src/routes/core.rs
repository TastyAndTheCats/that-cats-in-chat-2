//! All of the TCIC Admin site endpoint routes come through this module

use crate::template_data::{DashboardTemplate, IndexTemplate};
use actix_web::{
    get,
    web::{Path, ServiceConfig},
    Responder,
};

/// Not logged in
#[get("/")]
async fn index() -> impl Responder {
    IndexTemplate {
        login_url: "/auth/login".to_owned(),
    }
}

/// Logged in, here's your chatbot info
#[get("/dashboard/{owner_id}")]
async fn dashboard(path: Path<i32>) -> impl Responder {
    let owner_id = path.into_inner();
    DashboardTemplate {
        bot_login_url: format!("/auth/bot_login/{}", owner_id),
    }
}

// I always forget to register these routes when I create them until I go to test and they don't exist
// NBD, just something I'm aware of.
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index).service(dashboard);
}
