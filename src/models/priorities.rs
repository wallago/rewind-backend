use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status")]
#[sqlx(rename_all = "snake_case")]
pub enum Priorities {
    Low = 0,
    Medium = 1,
    High = 2,
}
