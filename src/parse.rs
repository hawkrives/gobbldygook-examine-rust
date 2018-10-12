extern crate serde;
extern crate serde_json;

use evaluate::AreaOfStudy;

pub fn parse(input: String) -> AreaOfStudy {
    serde_json::from_str(&input).unwrap()
}
