//! The Cats in Chat 2 Admin Site
//!
//! Provides access to the administration functions of TCIC via a web interface
//! This includes:
//! - Twitch account authentication
//!
//! This may end up being the backend site for administering TCIC to your own channel
//! But it's not what I want to use for the actual TCIC website

use actix_web::web::scope;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::dotenv;

mod routes; // Contains the different site parts
mod template_data;
mod tls; // Gets a ServerConfig with SSL certs attached // Contains the structs for the Askama templates

/// Runs a local website to manage various features of the chatbot / database
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info")); // Logging setup
    dotenv().expect(".env file not found"); // load environment variables from .env file

    HttpServer::new(|| {
        App::new()
            .service(scope("/auth").configure(routes::auth::config))
            .service(scope("").configure(routes::core::config)) // NOTE: since all routes start with /, you don't need the scope to be "/" here, but it must be last
            .wrap(Logger::default())
    })
    // .bind(("0.0.0.0", 8080))? // HTTP
    .bind_rustls_0_22(("0.0.0.0", 8443), tls::load_rustls_config().unwrap())? // HTTPS
    .run()
    .await
}
