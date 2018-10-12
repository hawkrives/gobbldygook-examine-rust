#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DayOfWeek {
    Mo,
    Tu,
    We,
    Th,
    Fr,
    Sa,
    Su,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Offering {
    pub day: DayOfWeek,
    pub start: String,
    pub end: String,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CourseStatus {
    #[serde(rename = "O")]
    Open,
    #[serde(rename = "C")]
    Closed,
    #[serde(rename = "X")]
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub clbid: String,
    pub crsid: String,
    pub credits: f32,
    pub department: String,
    pub enrolled: i32,
    pub groupid: Option<String>,
    pub grouptype: Option<String>,
    pub instructors: Vec<String>,
    pub max: i32,
    pub level: i32,
    pub number: i32,
    pub offerings: Vec<Offering>,
    pub notes: Option<Vec<String>>,
    pub pn: bool,
    pub revisions: Vec<serde_json::Value>,
    pub semester: i32,
    pub term: i32,
    pub year: i32,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub course_type: String,
    pub deptnum: String,
    pub status: CourseStatus,
}
