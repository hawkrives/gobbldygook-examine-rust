use crate::evaluate::{Course, CourseList, Requirement};
use crate::expression::{
    BooleanAndExpression, BooleanOrExpression, CourseExpression, HansonExpression,
    ReferenceExpression,
};

#[derive(Debug, Clone)]
pub struct ExpressionResult {
    pub matched_courses: Vec<Course>,
    pub success: bool,
}

fn expr_course(
    expression: CourseExpression,
    courses: CourseList,
    _dirty: Vec<Course>,
    _is_needed: bool,
) -> ExpressionResult {
    // TODO: … why does ExprCourse return a Vec of courses? Shouldn't it just return a single one?

    let matched_courses: Vec<Course> = courses.into_iter().filter(|c| c == &expression).collect();
    let success = matched_courses.len() > 0;

    ExpressionResult {
        matched_courses,
        success,
    }
}

fn expr_boolean_or(
    expression: BooleanOrExpression,
    children: &[Requirement],
    courses: CourseList,
    dirty: Vec<Course>,
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
        matched_courses,
        success: have_any_been_true,
    }
}

fn expr_boolean_and(
    expression: BooleanAndExpression,
    children: &[Requirement],
    courses: CourseList,
    dirty: Vec<Course>,
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
        matched_courses,
        success: have_all_been_true,
    }
}

fn expr_reference(expression: ReferenceExpression, children: &[Requirement]) -> ExpressionResult {
    let mut success = false;
    let mut matched_courses = vec![];

    if let Some(child) = children.iter().find(|&r| r.name == expression.requirement) {
        if let Some(detail) = &child.evaluated {
            success = detail.success;
            matched_courses = detail.matched_courses.clone();
        }
    }

    ExpressionResult {
        matched_courses,
        success,
    }
}

pub fn compute_expression(
    expression: HansonExpression,
    children: &[Requirement],
    courses: CourseList,
    dirty: Vec<Course>,
    _fulfillment: Option<Course>,
) -> ExpressionResult {
    let success = false;

    let default_result = ExpressionResult {
        matched_courses: vec![],
        success: success,
    };

    match expression {
        HansonExpression::Course(expr) => {
            // println!("{:?}", expr);
            expr_course(expr, courses, dirty, true)
        }
        HansonExpression::Of(_expr) => {
            // println!("{:?}", expr);
            default_result
        }
        HansonExpression::Reference(expr) => {
            // println!("{:?}", expr);
            expr_reference(expr, children)
        }
        HansonExpression::BooleanOr(expr) => {
            // println!("{:?}", expr);
            expr_boolean_or(expr, children, courses, dirty, true)
        }
        HansonExpression::BooleanAnd(expr) => {
            // println!("{:?}", expr);
            expr_boolean_and(expr, children, courses, dirty, true)
        }
        HansonExpression::Modifier(_expr) => {
            // println!("{:?}", expr);
            default_result
        }
        HansonExpression::Occurrence(_expr) => {
            // println!("{:?}", expr);
            default_result
        }
        HansonExpression::Where(_expr) => {
            // println!("{:?}", expr);
            default_result
        }
    }
}
