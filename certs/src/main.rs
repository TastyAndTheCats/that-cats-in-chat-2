use rcgen::{generate_simple_self_signed, CertifiedKey};
use std::fs;

fn main() {
    // Generate a certificate that's valid for "localhost" and "dev.thecatsinchat.com"
    let subject_alt_names = vec!["dev.thecatsinchat.com".to_string(), "localhost".to_string()];

    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();
    fs::write("certs/cert.pem", cert.pem()).expect("Unable to write certificate file");
    fs::write("certs/key.pem", key_pair.serialize_pem()).expect("Unable to write key file");
}
