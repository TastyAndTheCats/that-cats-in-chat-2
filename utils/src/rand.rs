use rand::distributions::Alphanumeric;
use rand::prelude::*;

pub fn generate_password(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
