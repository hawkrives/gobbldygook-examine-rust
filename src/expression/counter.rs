use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operator {
    Eq,
    Gte,
    Lte,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Shorthand {
    All,
    Any,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpressionCounter {
    pub operator: Operator,
    pub was: Option<Shorthand>,
    pub num: Option<u32>,
}
