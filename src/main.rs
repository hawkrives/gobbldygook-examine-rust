#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate serde_derive;

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
use std::fs;

fn main2() {
    let r = Requirement {
        name: "Requirement".to_string(),
        result: Some(HansonExpression::Course(CourseExpression {
            department: "CSCI".to_string(),
            number: 121,
        })),
        message: None,
        filter: Option::None,
        children_share_courses: false,
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
    let area = fs::read_to_string("/Users/rives/Projects/gobbldygook-examine-rust/sample.json")
        .expect("Unable to read file");

    // let area = r#"
    //     type: major
    //     name: Asian Studies
    //     revision: 2012-14

    //     result: Requirement & CSCI 121

    //     Requirement:
    //         Asian Studies:
    //             result: ASIAN 130

    //         Dance:
    //             result: DANCE 101

    //         result: all of (Asian Studies, Dance)
    // "#.to_string();

    parse(area);
}
