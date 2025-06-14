// use diesel::Queryable;
// use diesel::backend::Backend;
// use diesel::deserialize::{self, FromSql};
// use diesel::expression::AsExpression;
// use diesel::serialize::{self, Output, ToSql};
// use diesel::sql_types::Text;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, AsExpression)]
// #[diesel(sql_type = Text)]
// pub enum Status {
//     Todo,
//     InProgress,
//     Done,
// }

// impl Default for Status {
//     fn default() -> Self {
//         Self::Todo
//     }
// }

// impl<DB> ToSql<Text, DB> for Status
// where
//     DB: Backend,
//     str: ToSql<Text, DB>,
// {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
//         let s = match self {
//             Status::Todo => "Todo",
//             Status::InProgress => "InProgress",
//             Status::Done => "Done",
//         };
//         ToSql::<Text, DB>::to_sql(s, out)
//     }
// }

// impl<DB> FromSql<Text, DB> for Status
// where
//     DB: Backend,
//     String: FromSql<Text, DB>,
// {
//     fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
//         match String::from_sql(bytes)?.as_str() {
//             "Todo" => Ok(Status::Todo),
//             "InProgress" => Ok(Status::InProgress),
//             "Done" => Ok(Status::Done),
//             other => Err(format!("Unknown status: {}", other).into()),
//         }
//     }
// }

// impl<DB> Queryable<Text, DB> for Status
// where
//     DB: Backend,
//     Self: FromSql<Text, DB>,
// {
//     type Row = Self;

//     fn build(row: Self::Row) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
//         Ok(row)
//     }
// }
