use super::counter::ExpressionCounter;
use super::qualification::Qualification;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WhereExpression {
    pub qualification: Qualification,
    pub count: ExpressionCounter,
    pub distinct: bool,
}
