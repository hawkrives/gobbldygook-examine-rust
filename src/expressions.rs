extern crate serde;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CourseExpression {
    pub department: String,
    pub number: i16,
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
    pub num: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QualificationStaticValue {
    Number(i32),
    String(String),
}

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
    Max,
    Min,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualificationFunctionValue {
    pub name: QualificationFunctionName,
    pub prop: String,
    pub qualifier: Qualifier,
    pub computed: QualificationStaticValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QualificationValue {
    Static(QualificationStaticValue),
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
    pub key: String,
    pub value: QualificationValue,
    pub operator: QualificationOperator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Qualifier {
    Qualification,
    OrQualification,
    AndQualification,
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum FilterExpression {
    FilterOfExpression,
    FilterWhereExpression,
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
    // FilterExpression
}
