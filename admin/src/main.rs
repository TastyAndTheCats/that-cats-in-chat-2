use actix_web::web::scope;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod sites;
mod tls;

/// Runs a local website to manage various features of the chatbot / database
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init(); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file

    let tls_config = tls::load_rustls_config().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(scope("/").configure(sites::core::config))
            .service(scope("/bot").configure(sites::bot::auth::config))
    })
    // .bind(("0.0.0.0", 8080))?
    .bind_rustls_0_22(("0.0.0.0", 8443), tls_config)?
    .run()
    .await
}