extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct HansonInput {
    #[serde(rename = "type")]
    area_type: String,
    #[serde(rename = "name")]
    area_name: String,
    #[serde(rename = "revision")]
    area_revision: String,

    result: String,

    #[serde(flatten)]
    children: HashMap<String, HansonInputRequirement>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HansonInputRequirement {
    result: String,
    #[serde(flatten)]
    children: HashMap<String, HansonInputRequirement>,
}

pub fn parse(input: String) {
    let deserialized: HansonInput = serde_yaml::from_str(&input).unwrap();

    println!("---");
    println!("{:?}", deserialized);

    let serialized: String = serde_yaml::to_string(&deserialized).unwrap();

    println!("{}", serialized);

    let json: String = serde_json::to_string_pretty(&deserialized).unwrap();

    println!("---");
    println!("{}", json);
}
