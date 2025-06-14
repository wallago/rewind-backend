// use crate::schema::lists;
// use diesel::{AsChangeset, Insertable, Queryable};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Queryable, Serialize)]
// #[diesel(table_name = lists)]
// pub struct List {
//     pub id: i32,
//     pub board_id: i32,
//     pub name: String,
//     pub position: i32,
// }

// #[derive(Debug, Insertable, Deserialize)]
// #[diesel(table_name = lists)]
// pub struct NewList {
//     pub board_id: i32,
//     pub name: String,
//     pub position: i32,
// }

// #[derive(Debug, AsChangeset, Deserialize)]
// #[diesel(table_name = lists)]
// pub struct UpdateList {
//     pub name: Option<String>,
//     pub position: Option<i32>,
// }
