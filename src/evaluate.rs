use crate::compute::expression as compute_expression;
use crate::expressions::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type OverrideMap = BTreeMap<String, bool>;
pub type FulfillmentMap = BTreeMap<String, Course>;
pub type CourseList = Vec<Course>;

// the input to `evaluate`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AreaOfStudy {
    #[serde(rename = "type")]
    pub area_type: String,
    #[serde(rename = "name")]
    pub area_name: String,
    #[serde(rename = "revision")]
    pub area_revision: String,
    #[serde(rename = "slug")]
    pub area_url: Option<String>,
    pub result: HansonExpression,
    pub children: Vec<Requirement>,
}

// the output of `evaluate`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvaluationResult {
    pub area: AreaOfStudy,
    pub progress: (usize, usize),
    pub error: Option<String>,
    // pub matched_courses: Vec<Course>,
    pub success: bool,
    pub was_evaluated: bool,
    pub children_results: Vec<RequirementResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Requirement {
    pub name: String,
    pub result: Option<HansonExpression>,
    pub message: Option<String>,
    pub filter: Option<FilterExpression>,
    pub children_share_courses: Option<bool>,
    pub children: Vec<Requirement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequirementResult {
    pub applied_fulfillment: Option<Course>,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub was_evaluated: bool,
    pub overridden: bool,
    pub children_results: Vec<RequirementResult>,
    pub requirement: Requirement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpressionResult {
    pub expression: HansonExpression,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub was_evaluated: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Course {
    pub clbid: String,
    pub credits: f32,
    pub crsid: String,
    pub department: Vec<String>,
    pub groupid: Option<String>,
    pub grouptype: Option<String>,
    pub section: Option<String>,
    pub level: i32,
    pub number: i32,
    pub semester: i32,
    pub year: i32,
}

fn apply_filter(filter: FilterExpression, courses: CourseList) -> CourseList {
    courses
}

fn make_requirement_path(path: &Vec<String>) -> String {
    path.join("\x1C").to_lowercase()
}

fn apply_fulfillment_to_expression(
    result_expr: HansonExpression,
    fulfillment_value: Course,
) -> HansonExpression {
    result_expr
}

fn compute_requirement(
    requirement: Requirement,
    path: Vec<String>,
    mut courses: CourseList,
    overrides: OverrideMap,
    fulfillments: FulfillmentMap,
) -> RequirementResult {
    let req_name = requirement.name.clone();

    let mut path_to_here: Vec<String> = vec![];
    path_to_here.extend(path.iter().cloned());
    path_to_here.extend(vec![req_name]);

    let children_results: Vec<RequirementResult> = requirement
        .children
        .iter()
        .map(|req| {
            compute_requirement(
                req.clone(),
                path.clone(),
                courses.clone(),
                overrides.clone(),
                fulfillments.clone(),
            )
        }).collect();

    let mut success = false;
    let mut was_evaluated = false;
    let mut matched_courses: Vec<Course> = vec![];
    let mut applied_fulfillment: Option<Course> = None;

    let mut was_overridden = false;

    if let Some(mut result_expr) = requirement.result.clone() {
        was_evaluated = true;

        if let Some(filter) = requirement.filter {
            courses = apply_filter(filter, courses.clone());
        }

        // TODO: assert that requirement.result is not empty – probably in hanson-format, rather than examine-student

        let computed_result;

        let fulfillment = fulfillments.get(&make_requirement_path(&path));
        if let Some(value) = fulfillment {
            applied_fulfillment = Some(value.clone());
            result_expr = apply_fulfillment_to_expression(result_expr.clone(), value.clone());

            computed_result = compute_expression(
                result_expr,
                requirement.children.clone(),
                courses,
                vec![],
                Some(value.clone()),
            );
        } else {
            computed_result = compute_expression(
                result_expr,
                requirement.children.clone(),
                courses,
                vec![],
                None,
            );
        }

        success = computed_result.success;
        matched_courses = computed_result.matched_courses;

        let req_override = overrides.get(&make_requirement_path(&path));
        if let Some(value) = req_override {
            was_overridden = true;
            success = *value;
        }
    }

    RequirementResult {
        applied_fulfillment,
        matched_courses,
        success,
        was_evaluated,
        overridden: was_overridden,
        requirement: requirement.clone(),
        children_results,
    }
}

fn compute_progress(results: Vec<RequirementResult>) -> (usize, usize) {
    let successes: Vec<bool> = results
        .iter()
        .map(|r| r.success)
        .filter(|&pass| pass == true)
        .collect();

    (successes.len(), results.len())
}

pub fn evaluate_area(
    courses: CourseList,
    overrides: OverrideMap,
    fulfillments: FulfillmentMap,
    area_of_study: AreaOfStudy,
) -> EvaluationResult {
    // 1. Recursively call compute_requirement() on all children
    // 2. Compute this result

    let name = area_of_study.area_name.clone();
    let kind = area_of_study.area_type.clone();
    let path = vec![name, kind];

    let results: Vec<RequirementResult> = area_of_study
        .children
        .iter()
        .map(|req| {
            compute_requirement(
                req.clone(),
                path.clone(),
                courses.clone(),
                overrides.clone(),
                fulfillments.clone(),
            )
        }).collect();

    // let matched_courses = results.iter().map(|res| res.matched_courses).

    let result = compute_expression(
        area_of_study.result.clone(),
        area_of_study.children.clone(),
        courses,
        vec![],
        None,
    );

    let computed_result = result.success;

    let progress = compute_progress(results.clone());

    EvaluationResult {
        area: area_of_study,
        // matched_courses: vec![],
        success: computed_result,
        was_evaluated: true,
        children_results: results,
        error: None,
        progress,
    }
}
