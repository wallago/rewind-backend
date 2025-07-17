use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Tag {
    pub uuid: Uuid,
    pub board_uuid: Uuid,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize)]
pub struct NewTag {
    pub name: String,
    pub board_uuid: Uuid,
    pub color: String,
}
