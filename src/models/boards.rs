// use serde::{Deserialize, Serialize};

// #[derive(Debug, Queryable, Serialize)]
// #[diesel(table_name = tasks)]
// pub struct Board {
//     pub id: i32,
//     pub board_id: i32,
//     pub name: String,
//     pub position: i32,
// }

// #[derive(Debug, Insertable, Deserialize)]
// pub struct NewBoard {
//     pub name: String,
// }

// #[derive(Debug, AsChangeset, Deserialize)]
// pub struct UpdateBoard {
//     pub name: Option<String>,
// }
