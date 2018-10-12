use evaluate::Course;
use evaluate::CourseList;
use evaluate::ExpressionResult;
use evaluate::Requirement;
use expressions::*;

pub fn expression(
    expression: HansonExpression,
    filter: Option<FilterExpression>,
    children: Vec<Requirement>,
    courses: CourseList,
    dirty: Vec<Course>,
    fulfillment: Option<Course>,
) -> ExpressionResult {
    ExpressionResult {
        expression: expression,
        matched_courses: vec![],
        success: true,
        was_evaluated: true,
        overridden: false,
    }
}
