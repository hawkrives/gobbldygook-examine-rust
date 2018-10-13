#![allow(dead_code)]

mod compute;
mod evaluate;
mod expressions;
mod parse;

use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

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

    if opt.debug {
        println!("{}", serde_yaml::to_string(&area).unwrap());
    }

    if opt.debug {
        println!("---");
        println!("{}", serde_json::to_string_pretty(&area).unwrap());
    }

    let result = evaluate::evaluate_area(data.courses, data.overrides, data.fulfillments, area);

    println!("{}", serde_yaml::to_string(&result).unwrap());
}
