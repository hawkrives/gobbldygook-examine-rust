use evaluate::{Course as FullCourse, CourseList, ExpressionResult, Requirement};
use expressions::{CourseExpression, HansonExpression};

fn expr_course(
    expression: CourseExpression,
    courses: CourseList,
    dirty: Vec<FullCourse>,
    is_needed: bool,
) -> ExpressionResult {
    let success = false;
    let mut matched_courses = vec![];
    let was_evaluated = true;

    for c in courses {
        // println!("{:?} == {:?}", c.department, expression.department);
        if c == expression {
            // println!("success!");
            matched_courses.push(c);
        } else {
            // println!("aww...");
        }
    }

    ExpressionResult {
        expression: HansonExpression::Course(expression),
        was_evaluated,
        matched_courses,
        success,
    }
}

pub fn expression(
    expression: HansonExpression,
    children: Vec<Requirement>,
    courses: CourseList,
    dirty: Vec<FullCourse>,
    fulfillment: Option<FullCourse>,
) -> ExpressionResult {
    let success = false;

    let default_result = ExpressionResult {
        expression: expression.clone(),
        matched_courses: vec![],
        success: success,
        was_evaluated: true,
    };

    match expression.clone() {
        HansonExpression::Course(expr) => {
            println!("{:?}", expr);
            expr_course(expr, courses, dirty, true)
        }
        HansonExpression::Of(expr) => {
            println!("{:?}", expr);
            default_result
        }
        HansonExpression::Reference(expr) => {
            println!("{:?}", expr);
            default_result
        }
        HansonExpression::BooleanOr(expr) => {
            println!("{:?}", expr);
            default_result
        }
        HansonExpression::BooleanAnd(expr) => {
            println!("{:?}", expr);
            default_result
        }
        HansonExpression::Modifier(expr) => {
            println!("{:?}", expr);
            default_result
        }
        HansonExpression::Occurrence(expr) => {
            println!("{:?}", expr);
            default_result
        }
        HansonExpression::Where(expr) => {
            println!("{:?}", expr);
            default_result
        }
    }
}
