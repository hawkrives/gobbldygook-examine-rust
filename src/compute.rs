use evaluate::Course as FullCourse;
use evaluate::CourseList;
use evaluate::ExpressionResult;
use evaluate::Requirement;
use expressions::{FilterExpression, HansonExpression};

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn expression(
    expression: HansonExpression,
    filter: Option<FilterExpression>,
    children: Vec<Requirement>,
    courses: CourseList,
    dirty: Vec<FullCourse>,
    fulfillment: Option<FullCourse>,
) -> ExpressionResult {
    let success = false;

    match expression.clone() {
        HansonExpression::Course(expr) => {
            println!("{:?}", expr);
            ()
        }
        HansonExpression::Of(expr) => {
            println!("{:?}", expr);
            ()
        }
        HansonExpression::Reference(expr) => {
            println!("{:?}", expr);
            ()
        }
        HansonExpression::BooleanOr(expr) => {
            println!("{:?}", expr);
            ()
        }
        HansonExpression::BooleanAnd(expr) => {
            println!("{:?}", expr);
            ()
        }
        HansonExpression::Modifier(expr) => {
            println!("{:?}", expr);
            ()
        }
        HansonExpression::Occurrence(expr) => {
            println!("{:?}", expr);
            ()
        }
        HansonExpression::Where(expr) => {
            println!("{:?}", expr);
            ()
        }
    };

    ExpressionResult {
        expression: expression.clone(),
        matched_courses: vec![],
        success: success,
        was_evaluated: true,
    }
}
