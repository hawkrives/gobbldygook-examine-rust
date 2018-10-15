use super::course::CourseExpression;
use super::qualification::Qualification;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterOfExpression {
    pub distinct: bool,
    pub of: Vec<CourseExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterWhereExpression {
    pub distinct: bool,
    #[serde(rename = "where")]
    pub qualification: Qualification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum FilterExpression {
    #[serde(rename = "FilterOf")]
    Of(FilterOfExpression),

    #[serde(rename = "FilterWhere")]
    Where(FilterWhereExpression),
}
