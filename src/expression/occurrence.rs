use super::counter;
use super::course;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OccurrenceExpression {
    pub course: course::CourseExpression,
    pub count: counter::ExpressionCounter,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}
