//! Functions that use the rand crate

use rand::distributions::Alphanumeric;
use rand::prelude::*;

/// Generates a random String of length from alphanumerics
pub fn generate_password(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
