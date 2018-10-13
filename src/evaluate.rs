use crate::compute::compute_expression;
use crate::expressions::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type OverrideMap = BTreeMap<String, bool>;
pub type Fulfillment = Course;
pub type FulfillmentMap = BTreeMap<String, Fulfillment>;
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

    pub detail: Option<EvaluationResult>,
}

// the output of `evaluate`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvaluationResult {
    pub expression_result: ExpressionResult,

    pub progress: (usize, usize),
    pub error: Option<String>,

    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Requirement {
    /// The name of the requirement
    pub name: String,

    /// The expressions that make up the body of the requirement
    pub result: Option<HansonExpression>,
    pub message: Option<String>,
    pub filter: Option<FilterExpression>,

    /// The attributes of the requirement
    pub children_share_courses: Option<bool>,

    /// The children of the requirement
    pub children: Vec<Requirement>,

    /// The result of evaluating the `result` field
    pub detail: Option<RequirementResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequirementResult {
    pub applied_fulfillment: Option<Fulfillment>,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub overridden: bool,
    pub expression_result: ExpressionResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpressionResult {
    pub expression: HansonExpression,
    pub matched_courses: Vec<Course>,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Course {
    pub clbid: String,
    pub credits: ordered_float::OrderedFloat<f32>,
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

struct FilterByArgs {
    filtered: Vec<Course>,
    clause: Qualifier,
    distinct: bool,
    all_courses: Option<Vec<Course>>,
    counter: Option<ExpressionCounter>,
}

fn filter_by_qualification(
    filtered: Vec<Course>,
    clause: Qualification,
    distinct: bool,
    all_courses: Option<Vec<Course>>,
    counter: Option<ExpressionCounter>,
) -> Vec<Course> {
    let mut filtered = filtered.clone();

    let _computed_value: Option<i32> = None;

    if let QualificationValue::Function(func) = clause.clone().value {
        let _values = all_courses
            .unwrap_or(filtered.clone())
            .into_iter()
            /*.filter(filter_by_where_clause)*/
            .map(|c| c);
        match func.name {
            QualificationFunctionName::Max => {}
            QualificationFunctionName::Min => {}
        };
    }

    filtered = filtered.into_iter().filter(|c| *c == clause).collect();

    if let Some(counter) = counter {
        if let Some(num_to_take) = counter.num {
            match counter.operator {
                HansonCounterOperator::Lte | HansonCounterOperator::Eq => {
                    filtered = filtered.into_iter().take(num_to_take as usize).collect()
                }
                HansonCounterOperator::Gte => filtered = filtered,
            }
        }
    }

    if distinct {
        filtered.sort();
        filtered.dedup();
    }

    filtered
}

fn filter_by_where_clause(args: FilterByArgs) -> Vec<Course> {
    match args.clause {
        Qualifier::Single(clause) => filter_by_qualification(
            args.filtered,
            clause,
            args.distinct,
            args.all_courses,
            args.counter,
        ),
        Qualifier::BooleanAnd(clause) => {
            let mut filtered = args.filtered;
            for q in clause.values {
                filtered = filter_by_where_clause(FilterByArgs {
                    filtered: filtered,
                    clause: q,
                    distinct: args.distinct,
                    all_courses: args.all_courses.clone(),
                    counter: args.counter.clone(),
                });
            }
            filtered
        }
        Qualifier::BooleanOr(clause) => {
            let mut filtered = args.filtered;
            for q in clause.values {
                filtered.extend(filter_by_where_clause(FilterByArgs {
                    filtered: filtered.clone(),
                    clause: q,
                    distinct: args.distinct,
                    all_courses: args.all_courses.clone(),
                    counter: args.counter.clone(),
                }));
            }
            filtered.sort();
            filtered.dedup();
            filtered
        }
    }
}

fn apply_filter(filter: FilterExpression, courses: CourseList) -> CourseList {
    match filter {
        FilterExpression::Of(expr) => courses
            .into_iter()
            .filter(|c| expr.of.iter().any(|e| e == c))
            .collect(),
        FilterExpression::Where(expr) => filter_by_where_clause(FilterByArgs {
            filtered: courses,
            clause: expr.qualifier,
            counter: None,
            all_courses: None,
            distinct: false,
        }),
    }
}

fn make_requirement_path(path: &Vec<&str>) -> String {
    path.join("\x1C").to_lowercase()
}

fn apply_fulfillment_to_expression(
    result_expr: HansonExpression,
    _fulfillment_value: Course,
) -> HansonExpression {
    result_expr
}

fn compute_requirement(
    requirement: Requirement,
    path: Vec<&str>,
    mut courses: CourseList,
    overrides: OverrideMap,
    fulfillments: FulfillmentMap,
) -> Requirement {
    let req_name = requirement.name.clone();

    let mut path_to_here: Vec<&str> = vec![];
    path_to_here.extend(path.iter().cloned());
    path_to_here.push(&req_name);

    let children_results = requirement
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

    if let Some(mut result_expr) = requirement.result.clone() {
        let mut applied_fulfillment: Option<Course> = None;

        let mut was_overridden = false;
        let computed_result;

        if let Some(filter) = requirement.filter.clone() {
            courses = apply_filter(filter, courses.clone());
        }

        let fulfillment = fulfillments.get(&make_requirement_path(&path));
        if let Some(value) = fulfillment {
            applied_fulfillment = Some(value.clone());
            result_expr = apply_fulfillment_to_expression(result_expr.clone(), value.clone());

            computed_result = compute_expression(
                result_expr,
                &requirement.children,
                courses,
                vec![],
                Some(value.clone()),
            );
        } else {
            computed_result =
                compute_expression(result_expr, &requirement.children, courses, vec![], None);
        }

        let mut success = computed_result.success;
        let matched_courses = computed_result.clone().matched_courses;

        let req_override = overrides.get(&make_requirement_path(&path));
        if let Some(value) = req_override {
            was_overridden = true;
            success = *value;
        }

        return Requirement {
            detail: Some(RequirementResult {
                applied_fulfillment,
                matched_courses,
                success,
                overridden: was_overridden,
                expression_result: computed_result,
            }),
            children: children_results,
            ..requirement
        };
    }

    Requirement {
        detail: None,
        children: children_results,
        ..requirement
    }
}

fn compute_progress(results: &[Requirement]) -> (usize, usize) {
    let successes: Vec<bool> = results
        .iter()
        .map(|r| match &r.detail {
            Some(detail) => detail.success,
            None => false,
        }).filter(|&pass| pass == true)
        .collect();

    (successes.len(), results.len())
}

pub fn evaluate_area(
    courses: CourseList,
    overrides: OverrideMap,
    fulfillments: FulfillmentMap,
    area_of_study: AreaOfStudy,
) -> AreaOfStudy {
    // 1. Recursively call compute_requirement() on all children
    // 2. Compute this result

    let name = area_of_study.area_name.clone();
    let kind = area_of_study.area_type.clone();
    let path: Vec<&str> = vec![&name, &kind];

    let results: Vec<Requirement> = area_of_study
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
        &area_of_study.children,
        courses,
        vec![],
        None,
    );

    let computed_result = result.success;

    let progress = compute_progress(&results);

    AreaOfStudy {
        detail: Some(EvaluationResult {
            success: computed_result,
            expression_result: result,
            error: None,
            progress,
        }),
        ..area_of_study
    }
}
