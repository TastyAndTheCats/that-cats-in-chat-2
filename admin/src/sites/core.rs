use actix_web::{get, Responder};

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

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    // .service(echo)
    // .service(web::resource("/hey").route(web::get().to(manual_hello)));
}
