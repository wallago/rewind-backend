use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct List {
    pub uuid: Uuid,
    pub board_uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewList {
    pub board_uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Deserialize)]
pub struct UpdateList {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
}
