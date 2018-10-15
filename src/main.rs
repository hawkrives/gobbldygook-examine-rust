// #![feature(test)]

// extern crate test;

mod compute;
mod evaluate;
mod expressions;
mod parse;
mod print;

use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opts {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Activate debug mode
    #[structopt(long = "debug-area")]
    debug_area: bool,

    /// Serialize result
    #[structopt(long = "result")]
    serialize_result: bool,

    /// Output file
    #[structopt(name = "AREA", parse(from_os_str))]
    area_file: PathBuf,

    /// Output file
    #[structopt(name = "STUDENT", parse(from_os_str))]
    student_file: PathBuf,
}

fn main() {
    let opts = Opts::from_args();

    let area_buf = fs::read_to_string(opts.area_file).expect("Unable to read file");
    let student_buf = fs::read_to_string(opts.student_file).expect("Unable to read file");

    let area = parse::parse_area(area_buf);
    let data = parse::parse_student(student_buf);

    // println!("{:?}", data);

    if opts.debug_area {
        println!("---");
        println!("{:?}", area);
    }

    if opts.debug {
        println!("{}", serde_yaml::to_string(&area).unwrap());
    }

    if opts.debug {
        println!("---");
        println!("{}", serde_json::to_string_pretty(&area).unwrap());
    }

    let result = evaluate::evaluate_area(data.courses, data.overrides, data.fulfillments, area);

    if opts.serialize_result {
        println!("{}", serde_yaml::to_string(&result).unwrap());
    }

    print::print_area(result);
}

// #[cfg(test)]
// mod tests {
//     use self::test::Bencher;
//     use super::*;

//     #[bench]
//     fn bench_evaluate(b: &mut Bencher) {
//         let area_buf =
//             fs::read_to_string("/Users/rives/Projects/gobbldygook-examine-rust/asian.json")
//                 .expect("Unable to read file");
//         let student_buf =
//             fs::read_to_string("/Users/rives/Projects/gobbldygook-examine-rust/hawken.yaml")
//                 .expect("Unable to read file");

//         let area = parse::parse_area(area_buf);
//         let data = parse::parse_student(student_buf);

//         b.iter(|| {
//             let data = data.clone();
//             let area = area.clone();
//             evaluate::evaluate_area(data.courses, data.overrides, data.fulfillments, area)
//         })
//     }
// }
