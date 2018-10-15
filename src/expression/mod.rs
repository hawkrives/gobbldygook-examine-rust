use serde_derive::{Deserialize, Serialize};

mod boolean;
pub mod counter;
pub mod course;
pub mod filter;
mod modifier;
mod occurrence;
mod of;
pub mod qualification;
mod reference;
mod where_expr;

pub use self::boolean::{BooleanAndExpression, BooleanOrExpression};
pub use self::course::CourseExpression;
pub use self::modifier::ModifierExpression;
pub use self::occurrence::OccurrenceExpression;
pub use self::of::OfExpression;
pub use self::reference::ReferenceExpression;
pub use self::where_expr::WhereExpression;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum HansonExpression {
    BooleanAnd(BooleanAndExpression),
    BooleanOr(BooleanOrExpression),
    Course(CourseExpression),
    Modifier(ModifierExpression),
    Occurrence(OccurrenceExpression),
    Of(OfExpression),
    Reference(ReferenceExpression),
    Where(WhereExpression),
}
