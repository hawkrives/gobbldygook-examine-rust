use crate::evaluate::{AreaOfStudy, Requirement};
use crate::expression::HansonExpression;
use crate::parse::DataStruct;
// use crate::expressions;

pub fn print_student(data: &DataStruct) {
    println!("Available courses:");

    for c in data.courses.clone() {
        println!("{}", c);
    }

    println!("");
}

pub fn print_area(area_of_study: AreaOfStudy) {
    println!("Name: {}", area_of_study.area_name);
    println!("Type: {}", area_of_study.area_type);
    println!("Revision: {}", area_of_study.area_revision);

    if let Some(detail) = area_of_study.evaluated {
        println!(
            "Status: {}",
            if detail.success { "Success" } else { "Failure" }
        );

        let (at, of) = detail.progress;
        println!("Progress: {} of {}", at, of);
    } else {
        println!("Status: Not Evaluated");
    }

    println!("");
    println!("## Requirements ##");
    println!("");

    for child in area_of_study.children {
        print_requirement(child);
        println!("");
    }
}

fn print_requirement(req: Requirement) {
    println!("Requirement: {}", req.name);

    if let Some(detail) = req.evaluated {
        println!("Status: {}", detail.success);
    }

    if let Some(result) = req.result {
        println!("Detail:\n{}", print_expression(result));
    }

    if !req.children.is_empty() {
        println!("");
        println!("## Inner Requirements ##");

        for child in req.children {
            print_requirement(child);
        }

        println!("");
    }
}

fn print_expression(expr: HansonExpression) -> String {
    match expr {
        HansonExpression::Course(course) => {
            format!("(course {} {})", course.department.join("/"), course.number)
        }
        HansonExpression::Of(of) => format!(
            "have N of {}:\n\t{}",
            of.count.num.unwrap_or(0),
            of.of
                .into_iter()
                .map(print_expression)
                .collect::<Vec<String>>()
                .join("\n\t")
        ),
        HansonExpression::BooleanAnd(and) => format!(
            "\t& {}",
            and.values
                .into_iter()
                .map(print_expression)
                .collect::<Vec<String>>()
                .join("\n\t& ")
        ),
        HansonExpression::BooleanOr(or) => format!(
            "\t| {}",
            or.values
                .into_iter()
                .map(print_expression)
                .collect::<Vec<String>>()
                .join("\n\t| ")
        ),
        HansonExpression::Modifier(_) => "<a modifier>".to_string(),
        HansonExpression::Occurrence(_) => "<an occurrence>".to_string(),
        HansonExpression::Where(_) => "<a where>".to_string(),
        HansonExpression::Reference(reference) => {
            format!("<a reference to {}>", reference.requirement)
        }
    }
}
