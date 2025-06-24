use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status")]
#[sqlx(rename_all = "snake_case")]
pub enum Status {
    Todo = 0,
    InProgress = 1,
    Done = 2,
}
