use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Tag {
    pub uuid: Uuid,
    pub name: String,
    pub color: String,
}
