//! Functions that use the rand crate

use rand::{distributions::Alphanumeric, prelude::*, seq::SliceRandom, Rng};

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

pub fn usize_random_number_1_to(num: usize) -> usize {
    if num > 1 {
        rand::thread_rng().gen_range(1..num + 1)
    } else {
        num
    }
}

pub fn random_number_x_to_y(x: i32, y: i32) -> i32 {
    if x < y {
        rand::thread_rng().gen_range(x..y + 1)
    } else if y < x {
        rand::thread_rng().gen_range(y..x + 1)
    } else {
        // x == y
        x
    }
}

pub fn random_from_vec<T>(input: &Vec<T>) -> Option<&T> {
    input.choose(&mut rand::thread_rng())
}
