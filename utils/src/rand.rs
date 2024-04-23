//! Functions that use the rand crate

use rand::{distributions::Alphanumeric, prelude::*, Rng};

/// Generates a random String of length from alphanumerics
pub fn generate_password(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

// random range from 0..
pub fn random_number_0_to(num: i32) -> i32 {
    rand::thread_rng().gen_range(0..num)
}

pub fn random_number_1_to(num: i32) -> i32 {
    if num > 1 {
        rand::thread_rng().gen_range(1..num + 1)
    } else {
        num
    }
}
