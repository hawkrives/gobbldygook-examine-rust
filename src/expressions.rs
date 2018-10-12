#[derive(Debug, Clone)]
pub struct CourseExpression {
    pub department: String,
    pub number: i16,
}

#[derive(Debug, Clone)]
pub enum HansonCounterOperator {
    Eq,
    Gte,
    Lte,
}

#[derive(Debug, Clone)]
pub enum HansonCounterShorthand {
    All,
    Any,
    None,
}

#[derive(Debug, Clone)]
pub struct ExpressionCounter {
    pub operator: HansonCounterOperator,
    pub was: Option<HansonCounterShorthand>,
    pub num: f32,
}

#[derive(Debug, Clone)]
pub enum QualificationStaticValue {
    Number(i32),
    String(String),
}

#[derive(Debug, Clone)]
pub struct QualificationBooleanOrValue {
    pub values: Vec<QualificationStaticValue>,
}

#[derive(Debug, Clone)]
pub struct QualificationBooleanAndValue {
    pub values: Vec<QualificationStaticValue>,
}

#[derive(Debug, Clone)]
pub enum QualificationFunctionName {
    Max,
    Min,
}

#[derive(Debug, Clone)]
pub struct QualificationFunctionValue {
    pub name: QualificationFunctionName,
    pub prop: String,
    pub qualifier: Qualifier,
    pub computed: QualificationStaticValue,
}

#[derive(Debug, Clone)]
pub enum QualificationValue {
    Static(QualificationStaticValue),
    BooleanOr(QualificationBooleanOrValue),
    BooleanAnd(QualificationBooleanAndValue),
    Function(QualificationFunctionValue),
}

#[derive(Debug, Clone)]
pub enum QualificationOperator {
    Lte,
    Lt,
    Eq,
    Gte,
    Gt,
    Neq,
}

#[derive(Debug, Clone)]
pub struct OrQualification {
    pub values: Vec<Qualifier>,
}

#[derive(Debug, Clone)]
pub struct AndQualification {
    pub values: Vec<Qualifier>,
}

#[derive(Debug, Clone)]
pub struct Qualification {
    pub key: String,
    pub value: QualificationValue,
    pub operator: QualificationOperator,
}

#[derive(Debug, Clone)]
pub enum Qualifier {
    Qualification,
    OrQualification,
    AndQualification,
}

#[derive(Debug, Clone)]
pub struct OfExpression {
    pub count: ExpressionCounter,
    pub of: Vec<HansonExpression>,
}

#[derive(Debug, Clone)]
pub struct ReferenceExpression {
    pub requirement: String,
}

#[derive(Debug, Clone)]
pub struct BooleanOrExpression {
    pub values: Vec<HansonExpression>,
}

#[derive(Debug, Clone)]
pub struct BooleanAndExpression {
    pub values: Vec<HansonExpression>,
}

#[derive(Debug, Clone)]
pub enum ModifierWhatEnum {
    Course,
    Credit,
    Department,
}

#[derive(Debug, Clone)]
pub struct ModifierWhereExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub qualification: Qualification,
}

#[derive(Debug, Clone)]
pub struct ModifierFilterExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
}

#[derive(Debug, Clone)]
pub struct ModifierFilterWhereExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub qualifier: Qualifier,
}

#[derive(Debug, Clone)]
pub struct ModifierChildrenExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub children: Vec<ReferenceExpression>,
}

#[derive(Debug, Clone)]
pub struct ModifierChildrenWhereExpression {
    pub count: ExpressionCounter,
    pub what: ModifierWhatEnum,
    pub besides: Option<CourseExpression>,
    pub children: Vec<ReferenceExpression>,
    pub qualifier: Qualifier,
}

#[derive(Debug, Clone)]
pub enum ModifierExpression {
    Where(ModifierWhereExpression),
    Filter(ModifierFilterExpression),
    FilterWhere(ModifierFilterWhereExpression),
    Children(ModifierChildrenExpression),
    ChildrenWhere(ModifierChildrenWhereExpression),
}

#[derive(Debug, Clone)]
pub struct OccurrenceExpression {
    pub course: CourseExpression,
    pub count: ExpressionCounter,
}

#[derive(Debug, Clone)]
pub struct WhereExpression {
    pub qualifier: Qualifier,
    pub count: ExpressionCounter,
    pub distinct: bool,
}

#[derive(Debug, Clone)]
pub struct FilterOfExpression {
    pub distinct: bool,
    pub of: Vec<CourseExpression>,
}

#[derive(Debug, Clone)]
pub struct FilterWhereExpression {
    pub distinct: bool,
    pub qualifier: Qualifier,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterExpression {
    FilterOfExpression,
    FilterWhereExpression,
}

#[derive(Debug, Clone)]
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
