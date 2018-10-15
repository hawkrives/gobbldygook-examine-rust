use super::counter::ExpressionCounter;
use super::HansonExpression;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfExpression {
    pub count: ExpressionCounter,
    pub of: Vec<HansonExpression>,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}
