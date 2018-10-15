use crate::compute::compute_expression;
use crate::expression::counter;
use crate::expression::course;
use crate::expression::filter::*;
use crate::expression::qualification;
use crate::expression::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt;

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

    pub evaluated: Option<AreaOfStudyEvaluation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AreaOfStudyEvaluation {
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
    pub evaluated: Option<RequirementEvaluation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequirementEvaluation {
    pub applied_fulfillment: Option<Fulfillment>,
    pub matched_courses: Vec<Course>,
    pub success: bool,
    pub overridden: bool,
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

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}[{},{}]",
            self.department.join("/"),
            self.number,
            self.year,
            self.semester
        )
    }
}

impl PartialEq<Course> for qualification::SingleQualification {
    fn eq(&self, _other: &Course) -> bool {
        false
    }
}

impl PartialEq<qualification::SingleQualification> for Course {
    fn eq(&self, _other: &qualification::SingleQualification) -> bool {
        false
    }
}

fn compare_departments(lhs_depts: &Vec<String>, rhs_depts: &Vec<String>) -> bool {
    let self_depts: HashSet<String> = lhs_depts.clone().into_iter().collect();
    let other_depts: HashSet<String> = rhs_depts.clone().into_iter().collect();
    let diff: Vec<&String> = self_depts.symmetric_difference(&other_depts).collect();

    diff.len() == 0
}

fn compare_courses(lhs: &course::CourseExpression, rhs: &Course) -> bool {
    if !compare_departments(&lhs.department, &rhs.department) {
        return false;
    }

    if lhs.number != rhs.number {
        return false;
    }

    if let Some(year) = lhs.year {
        if year == rhs.year {
            return false;
        }
    }

    if let Some(semester) = lhs.semester {
        if semester == rhs.semester {
            return false;
        }
    }

    true
}

impl PartialEq<Course> for course::CourseExpression {
    fn eq(&self, other: &Course) -> bool {
        compare_courses(&self, &other)
    }
}

impl PartialEq<course::CourseExpression> for Course {
    fn eq(&self, other: &course::CourseExpression) -> bool {
        compare_courses(&other, &self)
    }
}

fn filter_by_qualification(
    filtered: Vec<Course>,
    clause: qualification::SingleQualification,
    distinct: bool,
    all_courses: Option<Vec<Course>>,
    counter: Option<counter::ExpressionCounter>,
) -> Vec<Course> {
    let mut filtered = filtered.clone();

    let _computed_value: Option<i32> = None;

    if let qualification::QualificationValue::Function(func) = clause.clone().value {
        let _values = all_courses
            .unwrap_or(filtered.clone())
            .into_iter()
            /*.filter(filter_by_where_clause)*/
            .map(|c| c);
        match func.name {
            qualification::FunctionNameEnum::Max => {}
            qualification::FunctionNameEnum::Min => {}
        };
    }

    filtered = filtered.into_iter().filter(|c| *c == clause).collect();

    if let Some(counter) = counter {
        if let Some(num_to_take) = counter.num {
            match counter.operator {
                counter::Operator::Lte | counter::Operator::Eq => {
                    filtered = filtered.into_iter().take(num_to_take as usize).collect()
                }
                counter::Operator::Gte => filtered = filtered,
            }
        }
    }

    if distinct {
        filtered.sort();
        filtered.dedup();
    }

    filtered
}

fn filter_by_where_clause(
    filtered: Vec<Course>,
    clause: qualification::Qualification,
    distinct: bool,
    all_courses: Option<Vec<Course>>,
    counter: Option<counter::ExpressionCounter>,
) -> Vec<Course> {
    match clause {
        qualification::Qualification::Single(clause) => {
            filter_by_qualification(filtered, clause, distinct, all_courses, counter)
        }
        qualification::Qualification::BooleanAnd(clause) => {
            let mut filtered = filtered;
            for q in clause.values {
                filtered = filter_by_where_clause(
                    filtered,
                    q,
                    distinct,
                    all_courses.clone(),
                    counter.clone(),
                );
            }
            filtered
        }
        qualification::Qualification::BooleanOr(clause) => {
            let mut filtered = filtered;
            for q in clause.values {
                filtered.extend(filter_by_where_clause(
                    filtered.clone(),
                    q,
                    distinct,
                    all_courses.clone(),
                    counter.clone(),
                ));
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
        FilterExpression::Where(expr) => {
            filter_by_where_clause(courses, expr.qualification, false, None, None)
        }
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
            // ..requirement,
            children_share_courses: requirement.children_share_courses,
            filter: requirement.filter,
            message: requirement.message,
            name: requirement.name,
            result: requirement.result,
            evaluated: Some(RequirementEvaluation {
                applied_fulfillment,
                matched_courses,
                success,
                overridden: was_overridden,
            }),
            children: children_results,
        };
    }

    Requirement {
        children_share_courses: requirement.children_share_courses,
        filter: requirement.filter,
        message: requirement.message,
        name: requirement.name,
        result: requirement.result,
        evaluated: None,
        children: children_results,
    }
}

fn compute_progress(results: &[Requirement]) -> (usize, usize) {
    let successes: Vec<bool> = results
        .iter()
        .map(|r| match &r.evaluated {
            Some(evaluated) => evaluated.success,
            None => false,
        }).filter(|&pass| pass == true)
        .collect();

    (successes.len(), results.len())
}

pub fn evaluate_area(
    courses: &[Course],
    overrides: &OverrideMap,
    fulfillments: &FulfillmentMap,
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
                courses.to_vec(),
                overrides.clone(),
                fulfillments.clone(),
            )
        }).collect();

    // let matched_courses = results.iter().map(|res| res.matched_courses).

    let result = compute_expression(
        area_of_study.result.clone(),
        &area_of_study.children,
        courses.to_vec(),
        vec![],
        None,
    );

    let computed_result = result.success;

    let progress = compute_progress(&results);

    AreaOfStudy {
        area_name: area_of_study.area_name,
        area_revision: area_of_study.area_revision,
        area_type: area_of_study.area_type,
        area_url: area_of_study.area_url,
        result: area_of_study.result,
        children: results,
        evaluated: Some(AreaOfStudyEvaluation {
            success: computed_result,
            error: None,
            progress,
        }),
    }
}
