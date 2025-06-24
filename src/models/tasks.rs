use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{priorities::Priorities, status::Status};

#[derive(Serialize, FromRow)]
pub struct Task {
    pub uuid: Uuid,
    pub list_uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: Priorities,
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
    pub priority: Priorities,
    pub position: i32,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub status: Option<Status>,
    pub priority: Option<Priorities>,
}
