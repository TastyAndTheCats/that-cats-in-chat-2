//! Contains some helpful functions that made me go cross-eyed when they were left in place
//! Most of these modules are representative of a specific third-party crate (rand, url, etc.)
//! Functions in the lib file are data conversions

pub mod file;
pub mod message;
pub mod num;
pub mod rand;
pub mod serde_json;
pub mod twitch;
pub mod url;

/// Parses a String as an i32
/// This is kind of stupid because it's so short but it's annoying to write.
pub fn parse_id(id: String) -> i32 {
    id.parse::<i32>().unwrap().to_owned()
}
