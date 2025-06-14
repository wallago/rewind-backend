use chrono::{DateTime, Utc};
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
    Todo = 0,
    InProgress = 1,
    Done = 2,
}
