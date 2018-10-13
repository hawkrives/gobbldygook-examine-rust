use crate::evaluate::{Course as FullCourse, CourseList, ExpressionResult, Requirement};
use crate::expressions::{
    BooleanAndExpression, BooleanOrExpression, CourseExpression, HansonExpression,
    ReferenceExpression,
};

fn expr_course(
    expression: CourseExpression,
    courses: CourseList,
    _dirty: Vec<FullCourse>,
    _is_needed: bool,
) -> ExpressionResult {
    let success = false;
    let mut matched_courses = vec![];

    for c in courses {
        // let has_course = courses.contains(&c);
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
        matched_courses,
        success,
    }
}

fn expr_boolean_or(
    expression: BooleanOrExpression,
    children: &[Requirement],
    courses: CourseList,
    dirty: Vec<FullCourse>,
    _is_needed: bool,
) -> ExpressionResult {
    let mut matched_courses = vec![];
    let mut have_any_been_true = false;

    for expr in expression.clone().values {
        let result = compute_expression(expr, children, courses.clone(), dirty.clone(), None);

        matched_courses.extend_from_slice(&result.matched_courses);

        have_any_been_true = have_any_been_true || result.success;
    }

    ExpressionResult {
        expression: HansonExpression::BooleanOr(expression),
        matched_courses,
        success: have_any_been_true,
    }
}

fn expr_boolean_and(
    expression: BooleanAndExpression,
    children: &[Requirement],
    courses: CourseList,
    dirty: Vec<FullCourse>,
    _is_needed: bool,
) -> ExpressionResult {
    let mut matched_courses = vec![];
    let mut have_all_been_true = false;

    for expr in expression.clone().values {
        let result = compute_expression(expr, children, courses.clone(), dirty.clone(), None);

        matched_courses.extend_from_slice(&result.matched_courses);

        have_all_been_true = have_all_been_true && result.success;
    }

    ExpressionResult {
        expression: HansonExpression::BooleanAnd(expression),
        matched_courses,
        success: have_all_been_true,
    }
}

fn expr_reference(expression: ReferenceExpression, children: &[Requirement]) -> ExpressionResult {
    let mut success = false;
    let mut matched_courses = vec![];

    if let Some(child) = children.iter().find(|&r| r.name == expression.requirement) {
        if let Some(detail) = &child.detail {
            success = detail.success;
            matched_courses = detail.matched_courses.clone();
        }
    }

    ExpressionResult {
        expression: HansonExpression::Reference(expression),
        matched_courses,
        success,
    }
}

pub fn compute_expression(
    expression: HansonExpression,
    children: &[Requirement],
    courses: CourseList,
    dirty: Vec<FullCourse>,
    _fulfillment: Option<FullCourse>,
) -> ExpressionResult {
    let success = false;

    let default_result = ExpressionResult {
        expression: expression.clone(),
        matched_courses: vec![],
        success: success,
    };

    match expression {
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
            expr_reference(expr, children)
        }
        HansonExpression::BooleanOr(expr) => {
            println!("{:?}", expr);
            expr_boolean_or(expr, children, courses, dirty, true)
        }
        HansonExpression::BooleanAnd(expr) => {
            println!("{:?}", expr);
            expr_boolean_and(expr, children, courses, dirty, true)
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
