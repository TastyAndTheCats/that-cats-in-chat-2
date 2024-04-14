//! Loads the SSL certificate and key files
//!
//! If you're looking at this you might know more than me, and probably know what you want to change
//! This is all static, since the certs project generates the certs files within itself.

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io;
use std::io::BufReader;

/// Loads certs.pem from the certs project
fn load_cert_file() -> io::Result<Vec<CertificateDer<'static>>> {
    let file = File::open("certs/cert.pem")?;
    certs(&mut BufReader::new(file)).collect()
}

/// Loads key.pem from the certs project
fn load_private_keys() -> io::Result<PrivateKeyDer<'static>> {
    let file = File::open("certs/key.pem")?;
    private_key(&mut BufReader::new(file)).map(|key| key.unwrap())
}

/// Loads the SSL certificates (from the certs project) and returns a rustls::ServerConfig
pub fn load_rustls_config() -> Result<ServerConfig, Box<dyn std::error::Error + Send + Sync>> {
    Ok(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(load_cert_file()?, load_private_keys()?)
        .expect("bad certificate/key"))
}
