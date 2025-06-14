use chrono::{DateTime, Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Task {
    pub uuid: Uuid,
    pub list_uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub position: i32,
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub finish_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct NewTask {
    pub list_uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub position: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status")]
#[sqlx(rename_all = "snake_case")]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

// #[derive(Debug, AsChangeset, Deserialize)]
// #[diesel(table_name = tasks)]
// pub struct UpdateTask {
//     pub name: Option<String>,
//     pub description: Option<String>,
//     pub status: Option<Status>,
//     pub position: Option<i32>,
//     pub updated_at: Option<String>,
//     pub deadline: Option<String>,
//     pub start_date: Option<String>,
//     pub finish_date: Option<String>,
// }
