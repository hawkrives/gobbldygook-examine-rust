use super::HansonExpression;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanOrExpression {
    pub values: Vec<HansonExpression>,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanAndExpression {
    pub values: Vec<HansonExpression>,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}
