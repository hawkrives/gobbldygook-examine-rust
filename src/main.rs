#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate structopt;
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

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

// extern crate clap;
// use clap::App;

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

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Activate debug mode
    #[structopt(long = "debug-area")]
    debug_area: bool,

    /// Output file
    #[structopt(name = "AREA", parse(from_os_str))]
    area_file: PathBuf,

    /// Output file
    #[structopt(name = "STUDENT", parse(from_os_str))]
    student_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);

    let area_buf = fs::read_to_string(opt.area_file).expect("Unable to read file");
    let student_buf = fs::read_to_string(opt.student_file).expect("Unable to read file");

    let area = parse::parse_area(area_buf);
    let data = parse::parse_student(student_buf);

    // println!("{:?}", data);

    if opt.debug_area {
        println!("---");
        println!("{:?}", area);
    }

    let serialized: String = serde_yaml::to_string(&area).unwrap();

    if opt.debug {
        println!("{}", serialized);
    }

    let json: String = serde_json::to_string_pretty(&area).unwrap();

    if opt.debug {
        println!("---");
        println!("{}", json);
    }

    let result = evaluate::evaluate_area(data.courses, data.overrides, data.fulfillments, area);

    // println!("{}", serde_yaml::to_string(&result).unwrap());
}
