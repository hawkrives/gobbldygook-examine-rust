#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::BTreeMap as Map;

mod expressions;
use expressions::*;

#[derive(Debug, Clone)]
pub struct AreaOfStudy {
    pub area_name: String,
    pub area_type: String,
    pub area_revision: String,
    pub requirements: Vec<Requirement>,
    pub result: HansonExpression,
}

#[derive(Debug, Clone)]
struct EvaluationResult<'a> {
    pub area: &'a AreaOfStudy,
    pub progress: (i32, i32),
    pub error: Option<String>,
    pub results: RequirementResult,
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub name: String,
    pub result: HansonExpression,
    pub filter: Option<FilterExpression>,
    pub computed: bool,
    pub children_share_courses: bool,
    pub children: Map<String, Requirement>,
    // pub overridden: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct RequirementResult {
    pub applied_fulfillment: Option<CourseExpression>,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub was_evaluated: bool,
    pub did_count: bool, // what does this do?
    pub children_results: Map<String, RequirementResult>,
    pub requirement: Requirement,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub clbid: String,
}

type OverrideMap = Map<String, bool>;
type FulfillmentMap = Map<String, Course>;
type CourseList = Vec<Course>;

fn compute_inner(
    requirement: &Requirement,
    path: Vec<String>,
    courses: &CourseList,
    overrides: &OverrideMap,
    fulfillments: &FulfillmentMap,
) -> RequirementResult {
    let r = Requirement {
        name: "Requirement".to_string(),
        result: HansonExpression::Course(CourseExpression {
            department: "CSCI".to_string(),
            number: 121,
        }),
        filter: Option::None,
        computed: true,
        children_share_courses: false,
        children: Map::new(),
    };

    let x = RequirementResult {
        applied_fulfillment: Option::None,
        matched_courses: vec![],
        success: true,
        was_evaluated: true,
        did_count: false, // what does this do?
        requirement: r,
        children_results: Map::new(),
    };

    return x;
}

fn compute(
    area_of_study: &AreaOfStudy,
    path: Vec<String>,
    courses: &CourseList,
    overrides: &OverrideMap,
    fulfillments: &FulfillmentMap,
) -> RequirementResult {
    let z: Vec<RequirementResult> = area_of_study
        .requirements
        .iter()
        .map(|req| compute_inner(req, vec![], courses, overrides, fulfillments))
        .collect();

    // let result = compute_chunk(area_of_study.result, vec![], courses, overrides, fulfillments);

    let r = Requirement {
        name: "Requirement".to_string(),
        result: HansonExpression::Course(CourseExpression {
            department: "CSCI".to_string(),
            number: 121,
        }),
        filter: Option::None,
        computed: true,
        children_share_courses: false,
        children: Map::new(),
    };

    let x = RequirementResult {
        applied_fulfillment: Option::None,
        matched_courses: vec![],
        success: true,
        was_evaluated: true,
        did_count: false, // what does this do?
        requirement: r,
        children_results: Map::new(),
    };

    return x;
}

fn compute_progress(evaluation_result: &RequirementResult) -> (i32, i32) {
    // match evaluation_result.requirement.success {
    //     HansonExpression::Course(expr) => expr,
    //     HansonExpression::Of(expr) => expr,
    //     HansonExpression::Reference(expr) => expr,
    // };

    if evaluation_result.success {
        return (1, 1);
    }

    (0, 1)
}

fn evaluate(
    courses: CourseList,
    overrides: OverrideMap,
    fulfillments: FulfillmentMap,
    area_of_study: &AreaOfStudy,
) -> EvaluationResult {
    let name = area_of_study.area_name.clone();
    let kind = area_of_study.area_type.clone();

    let result = compute(
        &area_of_study,
        vec![kind, name],
        &courses,
        &overrides,
        &fulfillments,
    );

    let progress = compute_progress(&result);

    return EvaluationResult {
        area: &area_of_study,
        progress: progress,
        error: Option::None,
        results: result,
    };
}

fn main() {
    println!("Hello, world!");

    let r = Requirement {
        name: "Requirement".to_string(),
        result: HansonExpression::Course(CourseExpression {
            department: "CSCI".to_string(),
            number: 121,
        }),
        filter: Option::None,
        computed: true,
        children_share_courses: false,
        children: Map::new(),
    };

    let v = vec![r];

    let a = AreaOfStudy {
        area_name: "Asian Studies".to_string(),
        area_type: "major".to_string(),
        area_revision: "2012-13".to_string(),
        requirements: v,
        result: HansonExpression::Reference(ReferenceExpression {
            requirement: "Requirement".to_string(),
        }),
    };

    let result = evaluate(vec![], Map::new(), Map::new(), &a);

    println!("{:#?}", result);
}
