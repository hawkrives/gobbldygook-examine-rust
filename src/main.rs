#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::BTreeMap;

mod expressions;
use expressions::*;

// the input to `evaluate`
#[derive(Debug, Clone)]
pub struct AreaOfStudy {
    pub area_name: String,
    pub area_type: String,
    pub area_revision: String,
    pub result: HansonExpression,
    pub requirements: Vec<Requirement>,
}

// the output of `evaluate`
#[derive(Debug, Clone)]
struct EvaluationResult {
    pub area: AreaOfStudy,
    pub progress: (i32, i32),
    pub error: Option<String>,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub was_evaluated: bool,
    pub did_count: bool, // what does this do?
    pub overridden: bool,
    pub children_results: Vec<RequirementResult>,
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub name: String,
    pub result: Option<HansonExpression>,
    pub message: Option<String>,
    pub filter: Option<FilterExpression>,
    pub children_share_courses: bool,
    pub children: Vec<Requirement>,
}

#[derive(Debug, Clone)]
pub struct RequirementResult {
    pub applied_fulfillment: Option<Course>,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub was_evaluated: bool,
    pub did_count: bool, // what does this do?
    pub overridden: bool,
    pub children_results: Vec<RequirementResult>,
    pub requirement: Requirement,
}

#[derive(Debug, Clone)]
pub struct ExpressionResult {
    pub expression: HansonExpression,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub was_evaluated: bool,
    pub did_count: bool, // what does this do?
    pub overridden: bool,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub clbid: String,
}

type OverrideMap = BTreeMap<String, bool>;
type FulfillmentMap = BTreeMap<String, Course>;
type CourseList = Vec<Course>;

fn compute_expression(
    expression: HansonExpression,
    filter: Option<FilterExpression>,
    children: Vec<Requirement>,
    courses: CourseList,
    dirty: Vec<Course>,
    fulfillment: Option<Course>,
) -> ExpressionResult {
    ExpressionResult {
        expression: expression,
        matched_courses: vec![],
        success: true,
        was_evaluated: true,
        did_count: true,
        overridden: false,
    }
}

fn apply_filter(filter: FilterExpression, courses: CourseList) -> CourseList {
    return courses;
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

    let did_count = false;
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
                requirement.filter,
                requirement.children.clone(),
                courses,
                vec![],
                Some(value.clone()),
            );
        } else {
            computed_result = compute_expression(
                result_expr,
                requirement.filter,
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
        applied_fulfillment: applied_fulfillment,
        matched_courses: matched_courses,
        success: success,
        was_evaluated: was_evaluated,
        did_count: did_count,
        overridden: was_overridden,
        requirement: requirement.clone(),
        children_results: children_results,
    }
}

fn compute_progress(evaluation_result: ExpressionResult) -> (i32, i32) {
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

fn evaluate_area(
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
        .requirements
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

    let result = compute_expression(
        area_of_study.result.clone(),
        None,
        area_of_study.requirements.clone(),
        courses,
        vec![],
        None,
    );

    let computed_result = result.success;

    let progress = compute_progress(result);

    EvaluationResult {
        area: area_of_study,
        matched_courses: vec![],
        success: computed_result,
        was_evaluated: true,
        did_count: false, // what does this do?
        overridden: false,
        children_results: vec![],
        error: None,
        progress: progress,
    }
}

fn main() {
    println!("Hello, world!");

    let r = Requirement {
        name: "Requirement".to_string(),
        result: Some(HansonExpression::Course(CourseExpression {
            department: "CSCI".to_string(),
            number: 121,
        })),
        message: None,
        filter: Option::None,
        children_share_courses: false,
        children: vec![],
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

    let result = evaluate_area(vec![], BTreeMap::new(), BTreeMap::new(), a);

    println!("{:#?}", result);
}
