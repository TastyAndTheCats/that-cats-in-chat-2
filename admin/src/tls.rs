use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io;
use std::io::BufReader;

fn load_cert_file(path: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    let file = File::open(path)?;
    certs(&mut BufReader::new(file)).collect()
}

fn load_private_keys(path: &str) -> io::Result<PrivateKeyDer<'static>> {
    let file = File::open(path)?;
    private_key(&mut BufReader::new(file)).map(|key| key.unwrap())
}

pub fn load_rustls_config() -> Result<ServerConfig, Box<dyn std::error::Error + Send + Sync>> {
    let config = ServerConfig::builder();

    // load TLS key/cert files
    let cert_chain = load_cert_file("certs/cert.pem")?;
    let private_key = load_private_keys("certs/key.pem")?;

    Ok(config
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .expect("bad certificate/key"))
}
