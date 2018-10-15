use serde_derive::{Deserialize, Serialize};
use serde_json;

type StaticValue = serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanOrValue {
    pub values: Vec<StaticValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanAndValue {
    pub values: Vec<StaticValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FunctionNameEnum {
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldNameEnum {
    #[serde(rename = "gereqs")]
    GeReq,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "department")]
    Department,
    #[serde(rename = "level")]
    Level,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionValue {
    pub name: FunctionNameEnum,
    pub prop: FieldNameEnum,
    pub qualifier: Box<Qualification>,
    pub computed_value: Option<StaticValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NumericValue {
    value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringValue {
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum QualificationValue {
    Number(NumericValue),
    String(StringValue),
    BooleanOr(BooleanOrValue),
    BooleanAnd(BooleanAndValue),
    Function(FunctionValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QualificationOperator {
    Lte,
    Lt,
    Eq,
    Gte,
    Gt,
    Neq,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrQualification {
    pub values: Vec<Qualification>,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AndQualification {
    pub values: Vec<Qualification>,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleQualification {
    pub key: FieldNameEnum,
    pub value: QualificationValue,
    pub operator: QualificationOperator,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Qualification {
    #[serde(rename = "Qualification")]
    Single(SingleQualification),
    BooleanOr(OrQualification),
    BooleanAnd(AndQualification),
}
