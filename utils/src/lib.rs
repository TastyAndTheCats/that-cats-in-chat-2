pub mod rand;
pub mod url;

pub fn parse_id(id: String) -> i32 {
    id.parse::<i32>().unwrap().to_owned()
}
