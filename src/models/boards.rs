use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Board {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewBoard {
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Deserialize)]
pub struct UpdateBoard {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
}
