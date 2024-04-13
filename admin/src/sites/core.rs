use actix_web::{get, web::Path, Responder};

use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    login_url: String,
}

#[get("/")]
async fn index() -> impl Responder {
    IndexTemplate {
        login_url: "/auth/login".to_owned(),
    }
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    bot_login_url: String,
}

#[get("/dashboard/{owner_id}")]
async fn dashboard(path: Path<i32>) -> impl Responder {
    let owner_id = path.into_inner();
    DashboardTemplate {
        bot_login_url: format!("/auth/bot_login/{}", owner_id),
    }
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(dashboard);
    // .service(web::resource("/hey").route(web::get().to(manual_hello)));
}
