use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CourseExpression {
    pub department: Vec<String>,
    pub number: i32,
    pub level: Option<i32>,
    pub semester: Option<i32>,
    pub year: Option<i32>,
}

#[cfg(test)]
mod test {
    use super::CourseExpression;
    use crate::evaluate::Course as FullCourse;

    #[test]
    fn courses_vs_course_exprs() {
        let CourseExpression = FullCourse {
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

        assert_eq!(yes_expr, CourseExpression);
    }

    #[test]
    fn courses_vs_course_exprs_diff_depts() {
        let CourseExpression = FullCourse {
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

        assert_ne!(no_expr, CourseExpression);
    }
}
