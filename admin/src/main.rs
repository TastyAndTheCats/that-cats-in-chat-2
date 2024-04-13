use actix_web::web::scope;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::dotenv;

mod sites;
mod tls;

/// Runs a local website to manage various features of the chatbot / database
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info")); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file

    let tls_config = tls::load_rustls_config().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(scope("/auth").configure(sites::auth::config))
            .service(scope("").configure(sites::core::config)) // NOTE: since all routes start with /, you don't need the scope to be "/" here, but it must be last
            .wrap(Logger::default())
    })
    // .bind(("0.0.0.0", 8080))?
    .bind_rustls_0_22(("0.0.0.0", 8443), tls_config)?
    .run()
    .await
}
