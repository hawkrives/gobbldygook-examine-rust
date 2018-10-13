use crate::evaluate::Course as FullCourse;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CourseExpression {
    pub department: Vec<String>,
    pub number: i32,
    pub level: Option<i32>,
    pub semester: Option<i32>,
    pub year: Option<i32>,
}

fn compare_departments(lhs_depts: &Vec<String>, rhs_depts: &Vec<String>) -> bool {
    let self_depts: HashSet<String> = lhs_depts.clone().into_iter().collect();
    let other_depts: HashSet<String> = rhs_depts.clone().into_iter().collect();
    let diff: Vec<&String> = self_depts.symmetric_difference(&other_depts).collect();

    diff.len() == 0
}

impl PartialEq<FullCourse> for CourseExpression {
    fn eq(&self, other: &FullCourse) -> bool {
        if !compare_departments(&self.department, &other.department) {
            return false;
        }

        if self.number != other.number {
            return false;
        }

        if let Some(year) = self.year {
            if year == other.year {
                return false;
            }
        }

        if let Some(semester) = self.semester {
            if semester == other.semester {
                return false;
            }
        }

        true
    }
}

impl PartialEq<CourseExpression> for FullCourse {
    fn eq(&self, other: &CourseExpression) -> bool {
        if !compare_departments(&self.department, &other.department) {
            return false;
        }

        if self.number != other.number {
            return false;
        }

        if let Some(other_semester) = other.semester {
            if self.semester == other_semester {
                return false;
            }
        }

        if let Some(other_year) = other.year {
            if self.year == other_year {
                return false;
            }
        }

        true
    }
}

#[test]
fn courses_vs_course_exprs() {
    let course = FullCourse {
        clbid: "1".to_string(),
        credits: ordered_float::OrderedFloat(1.0),
        crsid: "1".to_string(),
        department: vec!["CSCI".to_string()],
        groupid: Some("1".to_string()),
        grouptype: Some("R".to_string()),
        level: 100,
        number: 101,
        section: Some("A".to_string()),
        semester: 1,
        year: 2000,
    };

    let yes_expr = CourseExpression {
        department: vec!["CSCI".to_string()],
        level: None,
        number: 101,
        semester: None,
        year: None,
    };

    assert_eq!(yes_expr, course);
}

#[test]
fn courses_vs_course_exprs_diff_depts() {
    let course = FullCourse {
        clbid: "1".to_string(),
        credits: ordered_float::OrderedFloat(1.0),
        crsid: "1".to_string(),
        department: vec!["CSCI".to_string()],
        groupid: Some("1".to_string()),
        grouptype: Some("R".to_string()),
        level: 100,
        number: 101,
        section: Some("A".to_string()),
        semester: 1,
        year: 2000,
    };

    let no_expr = CourseExpression {
        department: vec!["ASIAN".to_string()],
        level: None,
        number: 101,
        semester: None,
        year: None,
    };

    assert_ne!(no_expr, course);
}

impl PartialEq<FullCourse> for Qualifier {
    fn eq(&self, _other: &FullCourse) -> bool {
        false
    }
}

impl PartialEq<Qualifier> for FullCourse {
    fn eq(&self, _other: &Qualifier) -> bool {
        false
    }
}

impl PartialEq<FullCourse> for Qualification {
    fn eq(&self, _other: &FullCourse) -> bool {
        false
    }
}

impl PartialEq<Qualification> for FullCourse {
    fn eq(&self, _other: &Qualification) -> bool {
        false
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HansonCounterOperator {
    Eq,
    Gte,
    Lte,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HansonCounterShorthand {
    All,
    Any,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpressionCounter {
    pub operator: HansonCounterOperator,
    pub was: Option<HansonCounterShorthand>,
    // #[serde(default)]
    pub num: Option<u32>,
}

type QualificationStaticValue = serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualificationBooleanOrValue {
    pub values: Vec<QualificationStaticValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualificationBooleanAndValue {
    pub values: Vec<QualificationStaticValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QualificationFunctionName {
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QualificationFunctionCourseFieldName {
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
pub struct QualificationFunctionValue {
    pub name: QualificationFunctionName,
    pub prop: QualificationFunctionCourseFieldName,
    // TODO: represent this without recursion?
    // pub qualifier: Qualifier,
    pub computed: Option<QualificationStaticValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualificationNumericValue {
    value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualificationStringValue {
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum QualificationValue {
    Number(QualificationNumericValue),
    String(QualificationStringValue),
    BooleanOr(QualificationBooleanOrValue),
    BooleanAnd(QualificationBooleanAndValue),
    Function(QualificationFunctionValue),
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
    pub values: Vec<Qualifier>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AndQualification {
    pub values: Vec<Qualifier>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Qualification {
    pub key: QualificationFunctionCourseFieldName,
    pub value: QualificationValue,
    pub operator: QualificationOperator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Qualifier {
    #[serde(rename = "Qualification")]
    Single(Qualification),
    BooleanOr(OrQualification),
    BooleanAnd(AndQualification),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfExpression {
    pub count: ExpressionCounter,
    pub of: Vec<HansonExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceExpression {
    pub requirement: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanOrExpression {
    pub values: Vec<HansonExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanAndExpression {
    pub values: Vec<HansonExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModifierWhatEnum {
    Course,
    Credit,
    Department,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierWhereExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub qualification: Qualification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierFilterExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierFilterWhereExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub qualifier: Qualifier,
}

// TODO: support "children: all"
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierChildrenExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub children: Vec<ReferenceExpression>,
}

// TODO: support "children: all"
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifierChildrenWhereExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub children: Vec<ReferenceExpression>,
    pub qualifier: Qualifier,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "from")]
pub enum ModifierExpression {
    Where(ModifierWhereExpression),
    Filter(ModifierFilterExpression),
    FilterWhere(ModifierFilterWhereExpression),
    Children(ModifierChildrenExpression),
    ChildrenWhere(ModifierChildrenWhereExpression),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OccurrenceExpression {
    pub course: CourseExpression,
    pub count: ExpressionCounter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WhereExpression {
    pub qualifier: Qualifier,
    pub count: ExpressionCounter,
    pub distinct: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterOfExpression {
    pub distinct: bool,
    pub of: Vec<CourseExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterWhereExpression {
    pub distinct: bool,
    pub qualifier: Qualifier,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum FilterExpression {
    #[serde(rename = "FilterOf")]
    Of(FilterOfExpression),
    #[serde(rename = "FilterWhere")]
    Where(FilterWhereExpression),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum HansonExpression {
    Course(CourseExpression),
    Of(OfExpression),
    Reference(ReferenceExpression),
    BooleanOr(BooleanOrExpression),
    BooleanAnd(BooleanAndExpression),
    Modifier(ModifierExpression),
    Occurrence(OccurrenceExpression),
    Where(WhereExpression),
}
