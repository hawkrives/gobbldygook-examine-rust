#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate serde_yaml;

mod evaluate;
mod expressions;
mod parse;

use evaluate::AreaOfStudy;
use evaluate::Requirement;
use expressions::CourseExpression;
use expressions::HansonExpression;
use expressions::ReferenceExpression;
use parse::parse;
use std::collections::BTreeMap;
use std::io::{self, Read};

fn main2() {
    let r = Requirement {
        name: "Requirement".to_string(),
        result: Some(HansonExpression::Course(CourseExpression {
            department: "CSCI".to_string(),
            number: 121,
        })),
        message: None,
        filter: Option::None,
        children_share_courses: Some(false),
        children: vec![],
    };

    let v = vec![r];

    let a = AreaOfStudy {
        area_name: "Asian Studies".to_string(),
        area_type: "major".to_string(),
        area_revision: "2012-13".to_string(),
        area_url: None,
        children: v,
        result: HansonExpression::Reference(ReferenceExpression {
            requirement: "Requirement".to_string(),
        }),
    };

    let result = evaluate::evaluate_area(vec![], BTreeMap::new(), BTreeMap::new(), a);

    println!("{:#?}", result);
}

fn main() {
    // let area = fs::read_to_string("./sample.json").expect("Unable to read file");
    let mut area = String::new();
    io::stdin()
        .read_to_string(&mut area)
        .expect("Unabled to read stdin");

    let deserialized = parse(area);

    println!("---");
    println!("{:?}", deserialized);

    let serialized: String = serde_yaml::to_string(&deserialized).unwrap();

    println!("{}", serialized);

    let json: String = serde_json::to_string_pretty(&deserialized).unwrap();

    println!("---");
    println!("{}", json);
}
