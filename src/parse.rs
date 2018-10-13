use serde_derive::{Deserialize, Serialize};

use crate::evaluate::AreaOfStudy;

pub fn parse_area(input: String) -> AreaOfStudy {
    serde_json::from_str(&input).unwrap()
}

use crate::evaluate::{CourseList, FulfillmentMap, OverrideMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataStruct {
    pub overrides: OverrideMap,
    pub courses: CourseList,
    pub fulfillments: FulfillmentMap,
}

pub fn parse_student(input: String) -> DataStruct {
    serde_yaml::from_str(&input).unwrap()
}
