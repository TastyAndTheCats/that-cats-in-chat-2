//! This is a standalone project sort of unrelated to the Chatbot at all.
//! It generates self-signed SSL certificates so we can locally fake HTTPS
//! so we don't have to do it last minute when we go to deploy it securely!
//!
//! When you run this binary it will create/overwrite ../cert.pem and ../key.pem
//! with a new self-signed SSL keypair for dev.thecatsinchat.com/localhost

use rcgen::{generate_simple_self_signed, CertifiedKey};
use std::fs;

fn main() {
    // Generate a certificate that's valid for "localhost" and "dev.thecatsinchat.com"
    let subject_alt_names = vec!["dev.thecatsinchat.com".to_string(), "localhost".to_string()];

    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();
    fs::write("certs/cert.pem", cert.pem()).expect("Unable to write certificate file");
    fs::write("certs/key.pem", key_pair.serialize_pem()).expect("Unable to write key file");
}
