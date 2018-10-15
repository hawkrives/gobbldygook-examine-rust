use super::counter::ExpressionCounter;
use super::course::CourseExpression;
use super::qualification::Qualification;
use super::reference::ReferenceExpression;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WhatEnum {
    Course,
    Credit,
    Department,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Where {
    pub count: ExpressionCounter,
    pub what: WhatEnum,
    pub besides: Option<CourseExpression>,
    pub qualification: Qualification,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Filter {
    pub count: ExpressionCounter,
    pub what: WhatEnum,
    pub besides: Option<CourseExpression>,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterWhere {
    pub count: ExpressionCounter,
    pub what: WhatEnum,
    pub besides: Option<CourseExpression>,
    pub qualification: Qualification,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

// TODO: support "children: all"
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Children {
    pub count: ExpressionCounter,
    pub what: WhatEnum,
    pub besides: Option<CourseExpression>,
    pub children: Vec<ReferenceExpression>,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

// TODO: support "children: all"
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildrenWhere {
    pub count: ExpressionCounter,
    pub what: WhatEnum,
    pub besides: Option<CourseExpression>,
    pub children: Vec<ReferenceExpression>,
    pub qualification: Qualification,

    pub matched_courses: Option<Vec<crate::evaluate::Course>>,
    pub result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "from")]
pub enum ModifierExpression {
    Where(Where),
    Filter(Filter),
    FilterWhere(FilterWhere),
    Children(Children),
    ChildrenWhere(ChildrenWhere),
}
